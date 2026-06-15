# Thin-film BLSS citation verification notes

The supplied BibTeX file is preserved at `citations/blss_thinfilm_refs.bib` and `source_material/new_thinfilm/blss_thinfilm_refs.bib`.

## Verification summary

A bibliographic sanity check was performed against publisher, university, or indexed records where available. The result is stored in `data/thinfilm/source_verification.csv`.

Key checks:

- `poughon2021`: DOI, title, journal, volume, and article number matched Frontiers/SCK CEN records.
- `garcia2021`: DOI, title, journal, volume, and article number matched Frontiers records.
- `perez2005`: DOI, title, journal, volume, and page range matched UAB portal/index records.
- `montras2009`: thesis title and author matched UAB full thesis PDF/record; no DOI was supplied.
- `polizzi2022`: DOI, title, journal, and article number matched PLOS/PMC records.
- `detrell2021`: DOI, title, journal, volume, and article number matched Frontiers records.
- `vermeulen2023`: DOI, title, journal, and article number matched Frontiers records.
- `blanken2014`: DOI and title matched PubMed/Wiley records.
- `blanken2016`: DOI, title, institution, and thesis type matched Wageningen University records.

`Schaap2017` and `esaC3` are retained in the BibTeX source material for future extensions but are not mapped to public Rust functions in this pack.
