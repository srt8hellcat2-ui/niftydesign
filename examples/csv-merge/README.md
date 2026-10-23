# Fill pages from CSV files (data merge and scripts)

Two examples for people who want to build documents from CSV data with NiftyDesign / DesignCraft. They drive the app through its control channel, so they run on Windows PowerShell with `designcraft.exe` (a DesignCraft 0.12 based build).

Both scripts start DesignCraft in the background, do the work, and close it again. Close other DesignCraft windows first. Your template file is never changed.

## 1. One label per CSV row: `csv-to-labels.ps1`

Uses DesignCraft's built-in data merge. Column headers in the first CSV row become fields; `{name}` in the template text is replaced with each row's value.

```powershell
.\csv-to-labels.ps1 -Csv people.csv -Template "Name: {name} from {city}" -Out labels.pdf
```

Sample: `people.csv` (3 rows gives 3 pages).

You can also do this by hand in the app: pick the CSV as the data source, insert fields into your label text, then run "Create Merged Document". The commands behind it are `data.source.select`, `data.placeholder.add` and `data.merge`.

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
