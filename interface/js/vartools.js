// command = {
//   name: '-someCommand',
//   description: '',
//   cmdOverride: '',
//   cmd: function () {
//     cmd = this.name;
//     for (let i = 0; i < this.arguments.length; i++) {
//       const arg = this.arguments[i];
//       cmd += " " + arg.value();
//     }
//     return cmd;
//   },
//   arguments: [
//     // Args go here
//   ]
// }
//
// text = {
//   name: 'a text input field',
//   description: '',
//   type: 'text',
//   id: uuid(),
//   value: function() {
//     return this.rawValue;
//   },
//   rawValue: '',
//   placeholder: '',
//   index: 0,
// }

// checkbox = {
//   name: 'a checkbox field',
//   description: '',
//   type: 'checkbox',
//   id: uuid(),
//   value: function () {
//     return this.rawValue;
//   },
//   rawValue: false,
//   index: 0
// }

// select = {
//   name: 'a select field',
//   description: '',
//   type: 'select',
//   id: uuid(),
//   value: function () {
//     return this.rawValue;
//   },
//   rawValue: "",
//   options: ["option 1", "option 2", "option 3"],
//   index: 0
// }

function uuid() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

vartools = {
  commands: [
    {
      name: "-BLS",
      description: "Perform the Box-Least Squares (BLS) transit search algorithm on the light curves",
      cmdOverride: "",
      cmd: function () {
        cmd = this.name;
        for (let i = 0; i < this.arguments.length; i++) {
          const arg = this.arguments[i];
          cmd += " " + arg.value();
        }
        return cmd;
      },
      arguments: [
        {
          name: "r",
          id: uuid(),
          value: function () {
            if (this.rawValue.length > 0) {
              return this.name + " " + this.rawValue;
            } else {
              return "";
            }
          },
          rawValue: "",
          placeholder: "rmin rmax",
          type: 'text',
          description: "Minimum and maximum stellar radius r to consider (in solar radii)",
          index: 0
        },
        {
          name: "q",
          id: uuid(),
          value: function () {
            if (this.rawValue.length > 0) {
              return this.name + " " + this.rawValue;
            } else {
              return "";
            }
          },
          rawValue: "",
          placeholder: "qmin qmax",
          type: 'text',
          description: "Fixed minimum and maximum q (fraction of orbit in transit) for the search",
          index: 0
        },
        {
          name: "density",
          id: uuid(),
          value: function() {
            if (this.rawValue.length > 0) {
              return this.name + " " + this.rawValue;
            } else {
              return "";
            }
          },
          rawValue: "",
          placeholder: "rho minexpdurfrac maxexpdurfrac",
          type: 'text',
          description: "stellar density (in grams per cubic centimeter) and a minimum and maximum fraction of the expected transit duration (assuming a circular orbit)",
          index: 0
        },
        {
          name: "minper",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "Minimum periods to search in days",
          index: 1
        },
        {
          name: "maxper",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "Maximum periods to search in days",
          index: 2
        },
        {
          name: "nfreq",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "Number of trial frequencies to scan",
          index: 3
        },
        {
          name: "nbins",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "Number of phase bins to break the light curve into (this should be at least 2/qmin)",
          index: 4
        },
        {
          name: "timezone",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "Number (in hours) to add to add to UTC to get the local time",
          index: 5
        },
        {
          name: "Npeak",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "number of peaks in the BLS spectrum to find and report",
          index: 6
        },
        {
          name: "outperiodogram",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: "set to 1 to output the BLS period vs. SN spectrum",
          index: 7
        },
        {
          name: "outdir",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: 'output directory for the BLS spectrum if outperiodogram is set to 1, ".bls" will be appended to the filename',
          index: 8
        },
        {
          name: "omodel",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: 'Set to 1 or zero that can be used to output the model for the light curve, the output directory is then given in modeloutdir, the suffix ".bls.model" will be appended to the filename.',
          index: 9
        },
        {
          name: "modeloutdir",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "",
          type: 'text',
          description: 'Output directory for omodel ".bls.model" files',
          index: 10
        },
        {
          name: "correctlc",
          id: uuid(),
          value: function () {
            if (this.selected) {
              return "1";
            } else {
              return "0";
            }
          },
          rawValue: "",
          placeholder: "",
          selected: false,
          type: 'checkbox',
          description: 'Will subtract the transit model from the light curve before passing it to the next command',
          index: 11
        },
        {
          name: "fittrap",
          id: uuid(),
          value: function () {
            if (this.selected) { return "fittrap" } else { return "" }
          },
          rawValue: "",
          selected: false,
          placeholder: "",
          type: 'checkbox',
          description: "fit a trapezoidal transit to each BLS pea",
          index: 12,
        },
        {
          name: "nobinnedrms",
          id: uuid(),
          value: function () {
            if (this.selected) { return "nobinnedrms" } else { return "" }
          },
          rawValue: "",
          placeholder: "",
          selected: false,
          type: 'checkbox',
          description: "Procedure runs faster, but the SN will tend to be suppressed for high significance detections",
          index: 13,
        },
        {
          name: "ophcurve",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "outdir phmin phmax phstep",
          type: 'text',
          description: "model phase curve will be output to a file in the directory outdir with suffix '.bls.phcurve'. It will be generated with phases between phmin and phmax and a uniform step size of phstep.",
          index: 14,
        },
        {
          name: "ojdcurve",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "outdir jdstep",
          type: 'text',
          description: "model light curve will be output to a file in the directory outdir with suffix '.bls.jdcurve'. The times will be between the first and last times in the light curve with a uniform step size of jdstep.",
          index: 15,
        },
        {
          name: "uniform steps",
          id: uuid(),
          value: function () {
            if (this.rawValue == "Uniform Frequency Steps (Default)") {
              return "";
            } else {
              return this.rawValue
            }
          },
          type: "select",
          options: ["Uniform Frequency Steps (Default)", "stepP", "steplogP"],
          rawValue: "Uniform Frequency Steps (Default)",
          placeholder: "",
          description: "By default the BLS spectrum is sampled at uniform frequency steps. To sample it at uniform steps in period or log(period) use the 'stepP' or 'steplogP' keyword.",
          index: 16,
        },
        {
          name: "adjust-qmin-by-mindt",
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "adjust-qmin-by-mindt",
          type: "text",
          description: "Adaptively set the minimum q value to the maximum of qmin or mindt*frequency where mindt is the minimum time difference between consecutive points in the light curve",
          index: 17,
        },
        {
          name: "reduce-nbins",
          id: uuid(),
          value: function () {
            if (this.selected) {
              return "reduce-nbins";
            } else {
              return "";
            }
          },
          rawValue: "",
          type: 'checkbox',
          selected: false,
          description: "Adaptively reduce the number of phase bins at each frequency such that there are no more than 2 bins to sample a transit of duration qmin",
          index: 18,
        },
        {
          name: "reportharmonics",
          id: uuid(),
          value: function () {
            if (this.selected) {
              return "reportharmonics";
            } else {
              return "";
            }
          },
          rawValue: "",
          selected: false,
          type: 'checkbox',
          description: "Report harmonic frequencies",
          index: 19,
        },
      ]
    }
  ]
}
