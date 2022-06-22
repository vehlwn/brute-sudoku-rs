const express = require("express");
const app = express();
const http = require("http");
app.use("/", express.static(__dirname + "/dist"));
const PORT = 5000;
http.createServer(app).listen(5000, "0.0.0.0", function () {
  console.log(`Server listening on ${PORT}`);
});

