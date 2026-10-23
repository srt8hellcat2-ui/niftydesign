# Fill pages from CSV files (data merge and scripts)

Two examples for people who want to build documents from CSV data with NiftyDesign / DesignCraft. They drive the app through its control channel, so they run on Windows PowerShell with `designcraft.exe` (a DesignCraft 0.12 based build).

Both scripts start DesignCraft in the background, do the work, and close it again. Close other DesignCraft windows first. Your template file is never changed.

## 1. One label per CSV row: `csv-to-labels.ps1`

Uses DesignCraft's built-in data merge. Column headers in the first CSV row become fields; `{name}` in the template text is replaced with each row's value.

```powershell
.\csv-to-labels.ps1 -Csv people.csv -Template "Name: {name} from {city}" -Out labels.pdf
```

Sample: `people.csv` (3 rows gives 3 pages).

The same thing should be possible by hand in the app (I have not clicked through the menus myself, see the end of this page): pick the CSV as the data source, insert fields into your label text, then run "Create Merged Document". The commands behind it are `data.source.select`, `data.placeholder.add` and `data.merge`.

## 2. Two CSV files in a set order: `streets-and-interventions.ps1`

For documents like this: street one's pages, then all of street one's interventions, then street two's pages, then its interventions, and so on. Data merge can't interleave two files, so this script adds the pages itself.

1. Make a template document with parent pages. Put text labels on them that contain `{field}` tokens (the CSV column names), e.g. `Street: {street}` on parent A, `Length: {length} m, district {district}` on parent B, `{date} - {work}` on parent C. Save it as a `.designcraft` file.
2. Put a column that exists in both CSV files (here `street`) so each intervention is linked to its street.
3. Run:

```powershell
.\streets-and-interventions.ps1 -Template template.designcraft -Streets streets.csv `
    -Interventions interventions.csv -Key street -StreetParents A,B -InterventionParent C -Out result
```

This writes `result.designcraft` and `result.pdf`. With the sample files (2 streets, 5 interventions) you get 9 pages: Oak Street (2 pages), its 3 interventions, Elm Avenue (2 pages), its 2 interventions.

How it works: for each page it inserts a page with the right parent (`layout.pages.insert`), makes the page's own copy of the parent's labels (`layout.overrideParentItems`, which returns their ids), and replaces the `{field}` text in each copy (`text.select`, `text.insert`).

Tested with single-sided pages (Document Setup: Facing Pages off). With facing pages, make sure each parent has its labels on the side(s) it is used on. Labels are replaced as plain text, so each label takes the formatting of its first character.

Run `designcraft-cli describe <command>` to see what any command accepts.
## What was tested, and what was not (please read)

These examples were written with the help of an AI assistant and then run on a real PC. NiftyDesign is an unofficial build, not an ArtCraft release.

**Tested** (Windows 11, NiftyDesign 0.12.0-nifty-alpha.3, only the small sample files in this folder):

- `csv-to-labels.ps1` with `people.csv`: 3 rows gave 3 pages with the right names and cities, and a PDF was exported.
- `streets-and-interventions.ps1` with a small template (parent pages A, B and C, single-sided pages): 2 streets and 5 interventions gave 9 pages in the right order, with the right text on every page.

**Not tested, so it may not work for you:**

- Your own real files and template. The template for the second script is not included, so you have to make your own (step 1 above).
- Facing-pages documents. In my first try, labels on the left-hand pages were not filled. I think my test template was the cause (its labels were on one side of the parent page only), but I have not confirmed that.
- Big CSV files. Every page takes several commands, so hundreds of streets will take a while. I did not time it.
- Accents, symbols or very long text. A label that is replaced takes the formatting of its first character, so mixed formatting inside one label is lost.
- Image, QR and link fields, and putting several labels on one page (data merge has options for this, I did not try them).
- The by-hand method in the app's menus. The app's command list does not say where data merge lives in the menus, so I can't tell you where to click. The commands in the scripts are what I actually ran.

If a {field} name in a label does not exist in the CSV, the text is left as it is. If PowerShell refuses to run the scripts, start them with `powershell -ExecutionPolicy Bypass -File .\script.ps1 ...`.

If something does not work, please open an issue with your command, the error text and what you expected.
