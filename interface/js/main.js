let app = new Vue({
  el: "#app",
  data: {
    apiRunning: false,
    inputFileFound: "",
    inputFile: "/Users/llamicron/Desktop/october.dat"
  },
  methods: {
    isApiRunning() {
      const Http = new XMLHttpRequest();
      const url = 'http://localhost:8000/api/running';
      Http.open("GET", url);
      Http.send();
      Http.onreadystatechange = (e) => {
        result = JSON.parse(Http.response);
        if (result.status == "ok") {
          this.apiRunning = true;
          console.log("connected successfully");
        } else {
          console.log("Could not connect to transits API, make sure it's running on localhost:8000");
        }
      }
    },

    testInputFileExists() {
      var xhr = new XMLHttpRequest();
      url = 'http://localhost:8000/api/file_exists'
      xhr.open("POST", url, true);
      xhr.setRequestHeader('Content-Type', 'application/json; charset=UTF-8');
      // send the collected data as JSON
      xhr.send(JSON.stringify({
        file_path: this.inputFile
      }));

      xhr.onloadend = () => {
        result = JSON.parse(xhr.responseText);
        if (result.status == "ok" && result.file_exists) {
          this.inputFileFound = "File exists";
        } else {
          this.inputFileFound = "File does not exist";
        }
      };


      // const Http = new XMLHttpRequest();
      // const url = 'http://localhost:8000/api/file_exists';
      // Http.open("POST", url);
      // Http.setRequestHeader("Content-type", "application/json");
      // Http.send(JSON.stringify(
      //   { "file_path": self.inputFile }
      // ));
      // Http.onreadystatechange = (e) => {
      //   result = JSON.parse(Http.response);
      //   if (result.status == "ok" && result.file_exists) {
      //     this.inputFileFound = "File exists";
      //   } else {
      //     this.inputFileFound = "File does not exist";
      //   }
      // }
    }
  },
  mounted() {
    this.isApiRunning();
  }
})
