from pywayland.server import Display, Listener
from wlroots.allocator import Allocator
from wlroots.backend import Backend, BackendType
from wlroots.renderer import Renderer
from wlroots.wlr_types import Compositor, SubCompositor
from wlroots.wlr_types import (
    Seat,
    XCursorManager,
    OutputLayout,
    Cursor,
    DataDeviceManager,
    Scene,
    XdgShell,
    idle_notify_v1,
    Output,
)
from wlroots.wlr_types.pointer import (
    PointerAxisEvent,
    PointerButtonEvent,
    PointerMotionAbsoluteEvent,
    PointerMotionEvent,
)
from wlroots.wlr_types.xdg_shell import XdgSurface, XdgSurfaceRole
from wlroots.wlr_types.cursor import WarpMode
from libs.tools.devices import KeyboardHandler
from wlroots.util.box import Box


class NocturaDE:
    def __init__(self):
        self.display = Display()
        self.backend = Backend(self.display)
        self.renderer = Renderer.autocreate(self.backend)
        self.renderer.init_display(self.display)
        self.allocator = Allocator.autocreate(self.backend, self.renderer)
        self.compositor = Compositor(self.display, 1, self.renderer)
        self.subcompositor = SubCompositor(self.display)
        self.deviceManager = DataDeviceManager(self.display)
        self.xdgShell = XdgShell(self.display)
        self.outputLayout = OutputLayout()
        self.cursor = Cursor(self.outputLayout)
        self.xcursorManager = XCursorManager(None, 24)
        self.seat = Seat(self.display, "seat0")
        self.scene = Scene()
        self.sceneLayout = self.scene.attach_output_layout(self.outputLayout)
        self.eventLoop = self.display.get_event_loop()
        self.idle_notify = idle_notify_v1.IdleNotifierV1(self.display)

        

        self.windows: list[Window] = []
        self.keyboards: list[KeyboardHandler] = []
        self.grabbedWindow: Window | None = None
        self.grabX = 0.0
        self.grabY = 0.0
        self.resize_edges = None
        self.grab_box: Box | None = None
        self.outputDevices: list[Output] = []

        self.cursor.motion_event.add(Listener(self.changeCursorPosition)) # TODO: METHOD
        self.cursor.button_event.add(Listener(self.cursorButtonHandeler)) # TODO: METHOD
        self.cursor.motion_absolute_event.add(Listener(self.changeCursorPositionAbsolute)) # TODO: METHOD
        self.cursor.axis_event.add(Listener(self.cursorScrollWheelHandeler)) # TODO: METHOD

        backend.new_output_event.add(Listener(self.newOutputDevice)) # TODO: METHOD
        backend.new_input_event.add(Listener(self.newInputDevice)) # TODO: METHOD
        xdg_shell.new_surface_event.add(Listener(self.newSurface)) # TODO: METHOD
        seat.request_set_cursor_event.add(Listener(self.cursorChange)) # TODO: METHOD
        seat.request_set_selection_event.add(Listener(self.clipHandle))# TODO: METHOD 
        self.eventLoop = self.display.get_event_loop()
        self.eventLoop.add_signal(
            serverCursorMotion, signal.SIGINT, self.killDisplay, self.display  # TODO: METHOD
        )
                
        self.socketName = self.display.add_socket()
        print("Socket:", self.socketName.decode())
