if (!Array.prototype.last) {
  Array.prototype.last = function () {
    return this[this.length - 1];
  };
};

function get(url, callback) {
  var xmlHttp = new XMLHttpRequest();
  xmlHttp.onreadystatechange = function () {
    if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
      callback(JSON.parse(xmlHttp.responseText));
    }
  }
  xmlHttp.open("GET", url, true);
  xmlHttp.send(null);
}

function post(url, data, callback) {
  var xmlHttp = new XMLHttpRequest();
  xmlHttp.open("POST", url, true);
  xmlHttp.setRequestHeader("Content-type", "application/json");

  xmlHttp.onreadystatechange = function () {
    if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
      callback(JSON.parse(xmlHttp.responseText));
    }
  }

  xmlHttp.send(JSON.stringify(data));
}

let app = new Vue({
  el: "#app",
  data: {
    apiRunning: false,
    result: {},
    inputFile: "",
    inputFiles: [],
    vartools: vartools,
    commandLocked: false,
    totalCommand: "vartools -l {infile} ",
    vartoolsOutput: "Nothing here yet... Run Vartools first",
    commands: [],
    flags: vartools.flags,
    loading: false,
    reformat_loading: false
  },

  watch: {
    inputFile: function() {
      // this.totalCommand = this.parseTotalCommand();
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
    reformat() {
      url = "http://localhost:8000/api/reformat";
      data = {
        infile: this.inputFile,
      }

      this.reformat_loading = true


      post(url, data, result => {
        if (result.status == "error") {
          console.log(result);
          message = result.reason;
          status = "danger"
        } else {
          message = "Reformatted";
          status = "success"
        }
        UIkit.notification(message, { "status": status });
        this.reformat_loading = false
      })
    },

    isApiRunning() {
      get("http://localhost:8000/api/running", (result) => {
        if (result.status == "ok") {
          this.apiRunning = true;
          return;
        }
      });
      this.apiRunning = false;
    },

    // Runs the vartools command
    run() {
      if (!this.inputFile) {
        UIkit.notification("Please select an input file", { "status": "warning" });
        return false;
      }

      this.loading = true;
      this.vartoolsOutput = "Vartools is running..."

      data = {
        infile: this.inputFile,
        cmd: this.totalCommand
      }
      url = 'http://localhost:8000/api/vartools'

      post(url, data, (result) => {
        console.log(result);
        this.loading = false;
        UIkit.notification("Results are ready!", { "status": "success" });

        this.vartoolsOutput = result.vartools;
      })
    },

    getInputFiles() {
      url = "http://localhost:8000/api/input_files";
      get(url, (result) => {
        this.inputFiles = result.files;
      });
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
      // fullCommand += " -l " + this.outDir() + "formatted_input/lc_list "
      fullCommand += " -l {infile} "

      this.commands.forEach(command => {
        fullCommand += command.cmd();
      });

      this.flags.forEach(flag => {
        fullCommand += " " + flag.value();
      })

      fullCommand = this.removeExtraWhitespace(fullCommand);
      // fullCommand = fullCommand.split("{outdir}").join(this.outDir() + "vartools/")
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

    setInterval(() => {
      this.isApiRunning();
    }, 4000);
  }
})
