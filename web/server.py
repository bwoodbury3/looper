from flask import Flask, render_template, request
import json
import threading

import looper
import file_io


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


@app.route("/api/save", methods=["POST"])
def save():
    """
    Save to a file.

    Params:
        data: The data to save.
        name: The filename.
    """
    data = request.json
    name = data["name"]
    content = json.dumps(data["data"])
    file_io.save_project(name, content)
    return "", 200


@app.route("/api/load", methods=["POST"])
def load():
    """
    Load from a file.

    Params:
        name: The filename.
    """
    data = request.json
    name = data["name"]
    print(f"Loading {name} from [{file_io.all_projects()}]")
    if name in file_io.all_projects():
        content = file_io.restore_project(name)
        return content, 200
    return "", 404


@app.route("/api/projects", methods=["GET"])
def get_projects():
    """
    Get all project names.
    """
    names = file_io.all_projects()
    return json.dumps({"projects": names}), 200


# Run flask
app.run(host="0.0.0.0", threaded=True, port=1080)
