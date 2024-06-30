from flask import Flask, render_template, request
import json
import threading

import looper


# Init flask
app = Flask(__name__)

# Init audio.
assert looper.init_audio()
looper.register_modules()
runner = looper.Runner()
runner_thread = None


@app.route("/", methods=["GET"])
def index():
    return render_template("index.html", message="hello")


@app.route("/api/play", methods=["POST"])
def play():
    global runner_thread

    # Don't start if it's already playing.
    if runner_thread is not None:
        print("Already playing")
        return "", 200

    # Build the request object
    data = request.json
    data_str = json.dumps(data)

    # Start playback.
    print("Triggering playback")
    runner_thread = threading.Thread(target=runner.run, args=(data_str,), daemon=True)
    runner_thread.start()

    return "", 200


@app.route("/api/stop", methods=["POST"])
def stop():
    global runner_thread

    # Go ahead and just call regardless.
    print("Stopping...")
    runner.stop()

    # Clean up the runner thread.
    if runner_thread is not None:
        runner_thread.join(1.0)
        runner_thread = None
    print("Stopped")

    return "", 200


# Run flask
app.run(host="0.0.0.0", threaded=True, port=1082)
