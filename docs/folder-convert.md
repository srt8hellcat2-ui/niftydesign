# Convert a folder of Publisher files to PDF

Use the command-line program to convert local `.pub` files from Publisher 2002 or newer. It uses the existing importer and PDF exporter, leaves the originals untouched, and makes no network requests. It keeps going after an individual file fails.

```sh
designcraft-cli convert-folder "My Publisher files" "Converted PDFs" --recursive
```

Without `--recursive`, it searches only the input folder. File extensions are case-insensitive. The output folder is created if needed; it must differ from the input folder. An output folder inside the input is excluded from the search. Symbolic links are not followed.

All PDFs go directly into the output folder. `Flyer.pub` becomes `Flyer.pdf`. Filenames are shortened and characters unsuitable for portable filenames are replaced with underscores. Reserved device names get a prefix. Duplicate names, including differences only in case, receive numbered suffixes such as `Flyer-2.pdf`. The report identifies each original relative path and its PDF name. Subfolders are not recreated and no editable native copies are written.

Existing PDFs are left unchanged and reported as failures. Existing `report.md` or `report.json` stops the run before conversion, preserving both reports. To replace existing regular output files and reports deliberately:

```sh
designcraft-cli convert-folder "My Publisher files" "Converted PDFs" --recursive --overwrite
```

Output symbolic links and directories are refused even with `--overwrite`. Choose a writable output folder whose path is valid UTF-8. Use `--` before folder paths beginning with a dash. Run `designcraft-cli convert-folder --help` for the full usage text.

## Read the reports

`report.md` and `report.json` contain the same results, warning and note wording, PDF names, page counts and summary. The summary counts all four results and the total pages successfully converted. An empty folder, or one without `.pub` files, gets both reports with zero counts and a clear message.

- **Converted, no warnings:** a PDF was saved, and the importer/exporter returned no warnings or notes. This does not measure fidelity.
- **Converted, check these things:** a PDF was saved, with the warnings and notes listed in plain words. Technical field identifiers are replaced with readable descriptions. A note alone belongs here too; it does not prove a visible problem.
- **Not supported yet:** the importer refused a Publisher 97, 98 or 2000 file. Open and re-save it in Publisher 2002 or newer, then try again.
- **Could not read:** the file was damaged, unreadable, not a Publisher file, or could not be converted or saved. The explanation distinguishes these cases. An existing PDF is kept, not counted as a conversion by this run.

The command exits with **0** if every discovered file converted, even with warnings or notes, and also for an empty folder. It exits with **1** if a file is unsupported, cannot be read, or an operation fails. It exits with **2** for bad arguments. If the output folder cannot be written or existing reports must be preserved, the explanation goes to the terminal; a new pair of reports cannot be written there.

## Limits

This command does not repair damaged files, support older Publisher formats, or improve the importer's layout fidelity. Fonts can be substituted and unsupported formatting can be lost. See [known limitations](KNOWN-LIMITATIONS.md). It offers no measured fidelity claim.

Each file runs in a separate process with a 120-second time limit and a 512 MiB input limit. Folder searches stop with an explanation after 100,000 file/error entries, one million directory entries, or 128 subfolder levels. Existing per-file worker limits can shorten exceptionally long warning lists or notes, with an omission marker. Report entries are written to disk as conversion proceeds, so warning details do not accumulate in memory. Enough output disk space is still required. The two final reports are separate files; a disk or permission failure while writing them can leave an incomplete pair.

The Linux tests cover ordinary and recursive runs, duplicate and unusual names, empty and invalid inputs, output protection, unreadable files, a sparse file over the size limit, a mixed five-file fixture folder and all 24 real-file samples. Windows paths and very large folders have not been tested.

Check every converted file against the original, because no warning does not prove a perfect copy.
