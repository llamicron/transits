// model:
// vartools = {
//   commands: [
//     {
//       name: "a name here",
//       description: "a description here",
//       arguments: [
//         {
//           name: "an argument name",
//           value: "1.0",
//           description: "some desc here",
//           required: false
//         },
//         {
//           name: "another argument name",
//           value: "2.0",
//           description: "some other desc here",
//           required: true
//         },
//       ]
//     }
//   ]
// }

function uuidv4() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

vartools = {
  commands: [
    {
      name: "BLS",
      description: "Perform the Box-Least Squares (BLS) transit search algorithm on the light curves",
      cmd: "",
      arguments: [
        {
          name: "minper",
          id: uuidv4(),
          value: "",
          description: "Minimum periods to search in days",
          required: true,
        },
        {
          name: "maxper",
          id: uuidv4(),
          value: "",
          description: "Maximum periods to search in days",
          required: true,
        },
        {
          name: "nfreq",
          id: uuidv4(),
          value: "",
          description: "Number of trial frequencies to scan",
          required: true,
        },
        {
          name: "nbins",
          id: uuidv4(),
          value: "",
          description: "Number of phase bins to break the light curve into (this should be at least 2/qmin)",
          required: true,
        },
        {
          name: "timezone",
          id: uuidv4(),
          value: "",
          description: "Number (in hours) to add to add to UTC to get the local time",
          required: true,
        },
        {
          name: "Npeak",
          id: uuidv4(),
          value: "",
          description: "number of peaks in the BLS spectrum to find and report",
          required: true,
        },
        {
          name: "outperiodogram",
          id: uuidv4(),
          value: "",
          description: "set to 1 to output the BLS period vs. SN spectrum",
          required: true,
        },
        {
          name: "outdir",
          id: uuidv4(),
          value: "",
          description: 'output directory for the BLS spectrum if outperiodogram is set to 1, ".bls" will be appended to the filename',
          required: false,
        },
        {
          name: "omodel",
          id: uuidv4(),
          value: "",
          description: 'Set to 1 or zero that can be used to output the model for the light curve, the output directory is then given in modeloutdir, the suffix ".bls.model" will be appended to the filename.',
          required: true,
        },
        {
          name: "modeloutdir",
          id: uuidv4(),
          value: "",
          description: 'Output directory for omodel ".bls.model" files',
          required: false,
        },
        {
          name: "correctlc",
          id: uuidv4(),
          value: "",
          description: '1 or 0. set to 1 it will subtract the transit model from the light curve before passing it to the next command',
          required: true,
        },
      ]
    }
  ]
}
