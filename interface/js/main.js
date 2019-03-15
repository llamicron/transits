const Http = new XMLHttpRequest();
const url = 'http://localhost:8000/api/running';
Http.open("GET", url);
Http.send();
Http.onreadystatechange = (e) => {
  result = JSON.parse(Http.response);
  if (result.status == "ok") {
    console.log("connected successfully");
  } else {
    console.log("Could not connect to transits API, make sure it's running on localhost:8000");
  }
}
