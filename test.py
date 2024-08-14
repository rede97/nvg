import contextlib, glfw, skia
from OpenGL import GL

WIDTH, HEIGHT = 640, 480

@contextlib.contextmanager
def glfw_window():
    if not glfw.init():
        raise RuntimeError('glfw.init() failed')
    glfw.window_hint(glfw.STENCIL_BITS, 8)
    # see https://www.glfw.org/faq#macos
    glfw.window_hint(glfw.CONTEXT_VERSION_MAJOR, 3)
    glfw.window_hint(glfw.CONTEXT_VERSION_MINOR, 2)
    glfw.window_hint(glfw.OPENGL_FORWARD_COMPAT, True)
    glfw.window_hint(glfw.OPENGL_PROFILE, glfw.OPENGL_CORE_PROFILE)
    window = glfw.create_window(WIDTH, HEIGHT, '', None, None)
    glfw.make_context_current(window)
    glfw.swap_interval(1)
    yield window
    glfw.terminate()

@contextlib.contextmanager
def skia_surface(window):
    context = skia.GrDirectContext.MakeGL()
    (fb_width, fb_height) = glfw.get_framebuffer_size(window)
    backend_render_target = skia.GrBackendRenderTarget(
        fb_width,
        fb_height,
        0,  # sampleCnt
        0,  # stencilBits
        skia.GrGLFramebufferInfo(0, GL.GL_RGBA8))
    surface = skia.Surface.MakeFromBackendRenderTarget(
        context, backend_render_target, skia.kBottomLeft_GrSurfaceOrigin,
        skia.kRGBA_8888_ColorType, skia.ColorSpace.MakeSRGB())
    assert surface is not None
    yield surface
    context.abandonContext()

with glfw_window() as window:
    GL.glEnable(GL.GL_BLEND)
    GL.glBlendFunc(GL.GL_SRC_ALPHA, GL.GL_ONE_MINUS_SRC_ALPHA)
    GL.glClear(GL.GL_COLOR_BUFFER_BIT)
    GL.glClearColor(0.3, 0.3, 0.5, 1.0)

    with skia_surface(window) as surface:
        lattice = skia.Matrix()
        lattice.setScale(4.0, 4.0)
        lattice.preRotate(45.0)
        path = skia.Path()
        path.setFillType(skia.kInverseEvenOdd )
        path.moveTo(100, 100)
        path.lineTo(300, 300)
        path.lineTo(400, 100)
        path.lineTo(100, 400)
        path.addCircle(300, 200, 100)
        paint = skia.Paint(AntiAlias=True)
        paint.setStyle(skia.Paint.kFill_Style)

        paint.setColor(skia.ColorWHITE)
        paint.setStrokeWidth(0.01)
        while (glfw.get_key(window, glfw.KEY_ESCAPE) != glfw.PRESS
            and not glfw.window_should_close(window)):
            glfw.poll_events()
            GL.glClear(GL.GL_COLOR_BUFFER_BIT)
            with surface as canvas:
                canvas.drawPath(path, paint)
            surface.flushAndSubmit()
            glfw.swap_buffers(window)