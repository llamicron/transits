let app = new Vue({
  el: "#app",
  data: {
    apiRunning: false,
    inputFileFound: false,
    inputFile: "/Users/llamicron/Desktop/october.dat",
    vartools: vartools,
    commandLocked: false,
    totalCommand: "",
    vartoolsOutput: "Nothing here yet... Run Vartools first",
    commands: [],
    flags: vartools.flags,
    loading: false,
  },

  components: {
    "results-table": ResultsTableComponent,
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

    // Posts to the API to see if the file exists
    testInputFileExists() {
      var xhr = new XMLHttpRequest();
      url = 'http://localhost:8000/api/file_exists'
      xhr.open("POST", url, true);
      xhr.setRequestHeader('Content-Type', 'application/json; charset=UTF-8');
      // Replace \ with /
      file = this.inputFile.split('\\').join('/');
      // send the collected data as JSON
      xhr.send(JSON.stringify({
        file_path: file
      }));

      xhr.onloadend = () => {
        result = JSON.parse(xhr.responseText);
        if (result.status == "ok" && result.file_exists) {
          this.inputFileFound = true;
        } else {
          this.inputFileFound = false;
        }
      };
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
      this.commands.push(Object.assign({}, command))
    },

    outDir () {
      pattern = new RegExp('.+?(?=\.)');
      return this.inputFile.split('.')[0] + "/";
    }
  },

  mounted() {
    this.isApiRunning();
    this.testInputFileExists();

    addEventListener('stop', () => {
      this.reIndexCommands();
    })
  }
})
