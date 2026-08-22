# Citing AeroCodex and its formula evidence

Software citation and scientific-source citation are separate obligations.

For the software, cite the exact tagged version when available. Before final publication, cite the exact Git commit instead of implying that a pending tag exists. The repository `CITATION.cff` supplies the software metadata used by GitHub and citation managers.

A minimum reproducible software citation records:

- AeroCodex version or full commit SHA;
- repository URL;
- retrieval date when citing a commit;
- crate, function, and Formula Registry ID used.

For each formula, also cite the original source named by its source-registry seed and validation record. Preserve the formula's validation status, domain, units, assumptions, and caveats. An AeroCodex citation does not replace citation of the underlying paper, standard, report, or dataset.

For the alpha release slice, start from `release/release-manifest.toml`, then follow the formula's `validation_record` and `documentation` links. The worked example in `docs/examples/m00-angle-conversion.md` demonstrates the complete trace.
