from flask import Flask, render_template, request
import json

app = Flask(__name__)


@app.route("/")
def index():
    return render_template("index.html", message="hello")


@app.route("/api/play", methods=["POST"])
def play():
    data = request.json

    # Write json to a file (kinda lame)
    with open("/tmp/playback.json", "w") as f:
        json.dump(data, f)

    # Run the playback controller.
    print("Playing!")

    return "", 200


app.run(host="0.0.0.0", port=1080)
