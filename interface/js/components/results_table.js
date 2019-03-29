let ResultsTableComponent = Vue.component('results-table', {
  props: ['results'],
  data: function () {
    return {}
  },

  watch: {
    results: function () {
      this.processResults();
    }
  },

  methods: {
    processResults() {
      data = this.results;
      chunks = data.split("\n\n");

      results =[];

      chunks.forEach(chunk => {
        run = {}
        items = chunk.split("\n");
        items.forEach(item => {
          if (item.indexOf("=") != -1) {
            pair = item.split("=")
            run[pair[0].trim()] = pair[1].trim();
          }
        });

        if (!Object.keys(run).length == 0) {
          results.push(run);
        }
      });
      return results
    }
  },

  template: `
    <pre>{{ results }}</pre>
  `
})
