if (!Array.prototype.last) {
  Array.prototype.last = function () {
    return this[this.length - 1];
  };
};

let app = new Vue({
  el: "#app",
  data: {
    apiRunning: false,
    // inputFile: "/Users/llamicron/Desktop/october.dat",
    inputFile: "",
    inputFiles: [],
    vartools: vartools,
    commandLocked: false,
    totalCommand: "",
    vartoolsOutput: "Nothing here yet... Run Vartools first",
    commands: [],
    flags: vartools.flags,
    loading: false,
  },

  watch: {
    inputFile: function() {
      this.testInputFileExists();
      this.totalCommand = this.parseTotalCommand();
    },

    commands: {
      deep: true,
      handler() {
        this.totalCommand = this.parseTotalCommand();
      }
    },

    flags: {
      deep: true,
      handler() {
        this.totalCommand = this.parseTotalCommand();
      }
    }
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
          console.log("Could not connect to astrotools API, make sure it's running on localhost:8000");
        }
      }
    },

    // Runs the vartools command
    run() {
      this.loading = true;
      this.vartoolsOutput = "Vartools is running..."
      var xhr = new XMLHttpRequest();
      url = 'http://localhost:8000/api/vartools';
      xhr.open("POST", url, true);
      xhr.setRequestHeader("Content-Type", "application/json; charset=UTF-8");
      xhr.send(JSON.stringify({
        infile: this.inputFile,
        cmd: this.totalCommand,
      }));

      xhr.onloadend = (result) => {
        response = JSON.parse(result.currentTarget.responseText);

        this.loading = false;
        UIkit.notification("Results are ready!", {"status": "success"});

        this.vartoolsOutput = response.vartools;
      }
    },

    getInputFiles() {
      url = "http://localhost:8000/api/input_files";

      var xmlHttp = new XMLHttpRequest();
      xmlHttp.onreadystatechange = () => {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
          this.inputFiles = JSON.parse(xmlHttp.responseText)['files'];
        }
      }
      xmlHttp.open("GET", url, true); // true for asynchronous
      xmlHttp.send(null);

    },

    removeCommand(id) {
      console.log(id);
      this.commands = this.commands.filter(x => x.id != id);
    },

    uuid() {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
      });
    },

    // Parses the full vartools command
    parseTotalCommand() {
      if (this.commandLocked) {
        return this.totalCommand;
      }

      // Call vartools
      fullCommand = "vartools";
      // Add the input index file
      fullCommand += " -l " + this.outDir() + "formatted_input/lc_list "

      this.commands.forEach(command => {
        fullCommand += command.cmd();
      });

      this.flags.forEach(flag => {
        fullCommand += " " + flag.value();
      })

      fullCommand = this.removeExtraWhitespace(fullCommand);
      fullCommand = fullCommand.split("{outdir}").join(this.outDir() + "vartools/")
      return fullCommand;
    },

    copyToClipboard(str) {
      const el = document.createElement('textarea');
      el.value = str;
      document.body.appendChild(el);
      el.select();
      document.execCommand('copy');
      document.body.removeChild(el);
      UIkit.notification("Command copied");
    },

    reIndexCommands() {
      console.log("reindexing");

      ul = document.getElementById("sortableCommandsList");

      for (let i = 0; i < ul.children.length; i++) {
        const li = ul.children[i];
        this.commands.filter(x => x.id == li.id)[0].index = i;
      }
    },

    removeExtraWhitespace(str) {
      return str.replace(/ +(?= )/g, '');
    },

    addCommand(command) {
      command.id = this.uuid();
      for (let i = 0; i < command.arguments.length; i++) {
        const arg = command.arguments[i];
        arg.id = this.uuid();
      }
      command.index = this.commands.length;
      copy = _.clone(command);
      this.commands.push(copy);
    },

    outDir () {
      pattern = new RegExp('.+?(?=\.)');
      return this.inputFile.split('.')[0] + "/";
    }
  },

  mounted() {
    this.isApiRunning();
    this.getInputFiles();

    addEventListener('stop', () => {
      this.reIndexCommands();
    })
  }
})
