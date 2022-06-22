import "bootstrap/dist/css/bootstrap.min.css";
import $ from "jquery";

window.onload = (_event) => {
  console.log("page is fully loaded");
};
$("#solveBtn").on("click", async function () {
  console.log("solve");
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

// postData("https://example.com/answer", { answer: 42 }).then((data) => {
//   console.log(data);
// });
