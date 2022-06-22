import "bootstrap/dist/css/bootstrap.min.css";
import $ from "jquery";

window.onload = (_event) => {
  const ROWS = 13;
  $("#inputTable").attr("rows", ROWS);
  $("#result").attr("rows", ROWS);
};

function clearFields() {
  $("#inputTable").val(
    `
. . .  . . .  . . .
. . .  . . .  . . .
. . .  . . .  . . .

. . .  . . .  . . .
. . .  . . .  . . .
. . .  . . .  . . .

. . .  . . .  . . .
. . .  . . .  . . .
. . .  . . .  . . .
    `.trim()
  );
  $("#result").val("");
}

$("#solveBtn").on("click", async function () {
  $("#clearBtn").prop("disabled", true);
  $("#solveBtn").prop("disabled", true);
  $("#solveBtn").text("Solving...");
  postData("/recursive_solver", {
    table: $("#inputTable").val(),
    output_format: "Indent",
  }).then((data) => {
    console.log(data);
    const ok = data["Ok"];
    const error = data["Error"];
    if (ok) {
      $("#result").val(ok.table);
    } else if (error) {
      $("#result").val("Error: " + error.msg);
    }
    $("#clearBtn").prop("disabled", false);
    $("#solveBtn").prop("disabled", false);
    $("#solveBtn").text("Solve");
  });
});

$("#clearBtn").on("click", async function () {
  clearFields();
});

$("#example1Btn").on("click", async function () {
  $("#inputTable").val(
    `
. . .  . . .  . . .
. . .  . . 3  . 8 5
. . 1  2 . .  . . .

. . .  5 . 7  . . .
. . 4  . . .  1 . .
. 9 .  . . .  . . .

5 . .  . . .  . 7 3
. . 2  . 1 .  . . .
. . .  . 4 .  . . 9
    `.trim()
  );
});

$("#example2Btn").on("click", async function () {
  $("#inputTable").val(
    `
5 3 .  . 7 .  . . .
6 . .  1 9 5  . . .
. 9 8  . . .  . 6 .

8 . .  . 6 .  . . 3
4 . .  8 . 3  . . 1
7 . .  . 2 .  . . 6

. 6 .  . . .  2 8 .
. . .  4 1 9  . . 5
. . .  . 8 .  . 7 9
    `.trim()
  );
});

async function postData(url = "", data = {}) {
  const response = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  return response.json();
}

clearFields();
