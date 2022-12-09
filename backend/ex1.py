from flask import Flask, request
from flask_cors import CORS

app = Flask(__name__)

CORS(app)

@app.route('/')

def home():
    return ("<h1>Hola</h1>")


if __name__ == '__main__':
    app.run()
