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

function boolToInt(bool) {
  if (bool) {
    return "1";
  } else {
    return "0";
  }
}

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
          description: "Minimum and maximum stellar radius r to consider (in solar radii)",
          type: 'text',
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
          index: 0,
        },
        {
          name: "q",
          description: "Fixed minimum and maximum q (fraction of orbit in transit) for the search",
          type: 'text',
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
          index: 0,
        },
        {
          name: "density",
          description: "stellar density (in grams per cubic centimeter) and a minimum and maximum fraction of the expected transit duration (assuming a circular orbit)",
          type: 'text',
          id: uuid(),
          value: function () {
            if (this.rawValue.length > 0) {
              return this.name + " " + this.rawValue;
            } else {
              return "";
            }
          },
          rawValue: "",
          placeholder: "rho minexpdurfrac maxexpdurfrac",
          index: 0,
        },
        {
          name: "minper",
          description: "Minimum periods to search in days",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "minper",
          index: 1,
        },
        {
          name: "maxper",
          description: "Maximum periods to search in days",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "maxper",
          index: 2,
        },
        {
          name: "nfreq",
          description: "Number of trial frequencies to scan",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "nfreq",
          index: 3,
        },
        {
          name: "nbins",
          description: "Number of phase bins to break the light curve into (this should be at least 2/qmin)",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "nbins",
          index: 4
        },
        {
          name: "timezone",
          description: "Number (in hours) to add to add to UTC to get the local time",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "timezone",
          index: 5
        },
        {
          name: "Npeak",
          description: "number of peaks in the BLS spectrum to find and report",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "Npeak",
          index: 6
        },
        {
          name: "outperiodogrom",
          description: "Select to output the BLS period vs. SN spectrum",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            return boolToInt(this.rawValue);
          },
          rawValue: false,
          index: 7,
        },
        {
          name: "outdir",
          description: 'output directory for the BLS spectrum if outperiodogram is selected, ".bls" will be appended to the filename',
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "outdir",
          index: 8,
        },
        {
          name: "omodel",
          description: 'Select to output the model for the light curve, the output directory is then given in modeloutdir (the next argument), the suffix ".bls.model" will be appended to the filename.',
          type: 'checkbox',
          id: uuid(),
          value: function () {
            return boolToInt(this.rawValue);
          },
          rawValue: false,
          index: 9,
        },
        {
          name: "modeloutdir",
          description: 'Output directory for omodel ".bls.model" files, if omodel is selected',
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "modeloutdir",
          index: 10
        },
        {
          name: "correctlc",
          description: "Will subtract the transit model from the light curve before passing it to the next command",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            return boolToInt(this.rawValue);
          },
          rawValue: false,
          index: 11,
        },
        {
          name: "fittrap",
          description: "fit a trapezoidal transit to each BLS pea",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            if (this.rawValue) {
              return "fittrap";
            } else {
              return "";
            }
          },
          rawValue: false,
          index: 12,
        },
        {
          name: "nobinnedrms",
          description: "Procedure runs faster, but the SN will tend to be suppressed for high significance detections",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            if (this.rawValue) {
              return "nobinnedrms";
            } else {
              return "";
            }
          },
          rawValue: false,
          index: 13,
        },
        {
          name: "ophcurve",
          description: "model phase curve will be output to a file in the directory outdir with suffix '.bls.phcurve'. It will be generated with phases between phmin and phmax and a uniform step size of phstep.",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "outdir phmin phmax phstep",
          index: 14,
        },
        {
          name: "ojdcurve",
          description: "model light curve will be output to a file in the directory outdir with suffix '.bls.jdcurve'. The times will be between the first and last times in the light curve with a uniform step size of jdstep.",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "outdir jdstep",
          index: 15,
        },
        {
          name: "uniform steps",
          description: "By default the BLS spectrum is sampled at uniform frequency steps. To sample it at uniform steps in period or log(period) use the 'stepP' or 'steplogP' keyword.",
          type: 'select',
          id: uuid(),
          value: function () {
            if (this.rawValue == "Uniform Frequency Steps (Default)") {
              return "";
            } else {
              return this.rawValue
            }
          },
          rawValue: "Uniform Frequency Steps (Default)",
          options: ["Uniform Frequency Steps (Default)", "stepP", "steplogP"],
          index: 16,
        },
        {
          name: "adjust-qmin-by-mindt",
          description: "Adaptively set the minimum q value to the maximum of qmin or mindt*frequency where mindt is the minimum time difference between consecutive points in the light curve",
          type: 'text',
          id: uuid(),
          value: function () {
            return this.rawValue;
          },
          rawValue: "",
          placeholder: "adjust-qmin-by-mindt",
          index: 17,
        },
        {
          name: "reduce-nbins",
          description: "Adaptively reduce the number of phase bins at each frequency such that there are no more than 2 bins to sample a transit of duration qmin",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            if (this.rawValue) {
              return "reduce-nbins";
            } else {
              return "";
            }
          },
          rawValue: false,
          index: 18,
        },
        {
          name: "reportharmonics",
          description: "Report harmonic frequencies",
          type: 'checkbox',
          id: uuid(),
          value: function () {
            if (this.rawValue) {
              return "reportharmonics";
            } else {
              return "";
            }
          },
          rawValue: false,
          index: 19,
        },
      ]
    }
  ]
}
