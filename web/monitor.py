from flask_socketio import emit, SocketIO
import time
import threading

import looper


class Monitor:
    """
    Monitors the runner and emits data at the defined interval.
    """

    def __init__(self, socketio: SocketIO, runner: looper.Runner, control_period=0.1):
        self.socketio = socketio
        self.runner = runner
        self.thread = None
        self.running = False
        self.control_period = control_period

    def run(self):
        """
        Run the thread
        """
        self.running = True
        self.thread = threading.Thread(target=self._loop, daemon=True)
        self.thread.start()

    def stop(self):
        """
        Stop the thread
        """
        self.running = False

    def _loop(self):
        """
        Loooooooooop
        """
        while self.running:
            runner_running = self.runner.is_running()
            current_measure = self.runner.current_measure()

            data = {
                "playing": runner_running,
                "current_measure": current_measure,
            }
            self.socketio.emit("playback_monitor", data)

            # Sleep
            time.sleep(self.control_period)
