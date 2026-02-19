from __future__ import annotations
from typing import TYPE_CHECKING

from pywayland.server import Listener
from typing import Any

from wlroots.wlr_types import InputDevice, Keyboard
from wlroots.wlr_types.keyboard import KeyboardKeyEvent
if TYPE_CHECKING:
    from libs.tools.NocturaDE import NocturaDE


class KeyboardHandler:
    def __init__(self,keyboard: Keyboard, input_device: InputDevice, server: NocturaDE):
        self.keyboard = keyboard
        self.input_device = input_device
        self.server = server

        keyboard.modifiers_event.add(Listener(self.keyboardHandleModifiers))
        keyboard.key_event.add(Listener(self.keyboardHandleKey))

    def keyboardHandleModifiers(self, listener: Listener, data: Any):
        self.server.send_modifiers(self.keyboard.modifiers, self.input_device)

    def keyboardHandleKey(self, listener: Listener, key_event: KeyboardKeyEvent):
        self.server.send_key(key_event, self.input_device)