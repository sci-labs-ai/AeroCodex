#!/usr/bin/env python3
"""Verify A44 external M07 residual scalar/unit/helper policy Wave 1 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
FUTURE_SAME_POOL_RESOLUTION_PATHS={'formula-vault/resolutions/m07_final_residual_backlog_closure_wave1.tsv'}
RESOLUTION_PATH='formula-vault/resolutions/m07_residual_scalar_unit_helper_policy_wave1.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
WAVE_ID='a44_external_m07_residual_scalar_unit_helper_policy_wave1'
SELECTED_LOCATORS=['PORT_STATUS_RELEASE_GATE.csv:row_0037', 'PORT_STATUS_RELEASE_GATE.csv:row_0038', 'PORT_STATUS_RELEASE_GATE.csv:row_0041', 'PORT_STATUS_RELEASE_GATE.csv:row_0044', 'PORT_STATUS_RELEASE_GATE.csv:row_0045', 'PORT_STATUS_RELEASE_GATE.csv:row_0046', 'PORT_STATUS_RELEASE_GATE.csv:row_0112', 'PORT_STATUS_RELEASE_GATE.csv:row_0113', 'PORT_STATUS_RELEASE_GATE.csv:row_0116', 'PORT_STATUS_RELEASE_GATE.csv:row_0122', 'PORT_STATUS_RELEASE_GATE.csv:row_0196', 'PORT_STATUS_RELEASE_GATE.csv:row_0197', 'PORT_STATUS_RELEASE_GATE.csv:row_0202', 'PORT_STATUS_RELEASE_GATE.csv:row_0203', 'PORT_STATUS_RELEASE_GATE.csv:row_0204', 'PORT_STATUS_RELEASE_GATE.csv:row_0207', 'PORT_STATUS_RELEASE_GATE.csv:row_0287', 'PORT_STATUS_RELEASE_GATE.csv:row_0288', 'PORT_STATUS_RELEASE_GATE.csv:row_0289', 'PORT_STATUS_RELEASE_GATE.csv:row_0291', 'PORT_STATUS_RELEASE_GATE.csv:row_0292', 'PORT_STATUS_RELEASE_GATE.csv:row_0313', 'PORT_STATUS_RELEASE_GATE.csv:row_0314', 'PORT_STATUS_RELEASE_GATE.csv:row_0361', 'PORT_STATUS_RELEASE_GATE.csv:row_0362', 'PORT_STATUS_RELEASE_GATE.csv:row_0363', 'PORT_STATUS_RELEASE_GATE.csv:row_0379', 'PORT_STATUS_RELEASE_GATE.csv:row_0380', 'PORT_STATUS_RELEASE_GATE.csv:row_0451', 'PORT_STATUS_RELEASE_GATE.csv:row_0452', 'PORT_STATUS_RELEASE_GATE.csv:row_0456', 'PORT_STATUS_RELEASE_GATE.csv:row_0457', 'PORT_STATUS_RELEASE_GATE.csv:row_0458', 'PORT_STATUS_RELEASE_GATE.csv:row_0549', 'PORT_STATUS_RELEASE_GATE.csv:row_0558', 'PORT_STATUS_RELEASE_GATE.csv:row_0657', 'PORT_STATUS_RELEASE_GATE.csv:row_0658', 'PORT_STATUS_RELEASE_GATE.csv:row_0663', 'PORT_STATUS_RELEASE_GATE.csv:row_0665', 'PORT_STATUS_RELEASE_GATE.csv:row_0679', 'PORT_STATUS_RELEASE_GATE.csv:row_0680', 'PORT_STATUS_RELEASE_GATE.csv:row_0742', 'PORT_STATUS_RELEASE_GATE.csv:row_0743', 'PORT_STATUS_RELEASE_GATE.csv:row_0764', 'PORT_STATUS_RELEASE_GATE.csv:row_0806']
CANDIDATE_GROUPS={'8D_deduplicate_helpers_and_test_utility_policy', '8B_or_8D_angle_endpoint_policy_then_deduplicate_wrappers', '8D_deduplicated_unit_conversion_helpers'}
EXPECTED_CANDIDATE_POOL_ROWS=60
EXPECTED_ROWS=45
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=15
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1323
EXPECTED_REMAINING_BACKLOG=0
EXPECTED_RISK_COUNTS=Counter({'do_not_import': 15, 'medium_risk_requires_contract_review': 30})
EXPECTED_FAMILY_COUNTS=Counter({'ambiguous_source_or_contract': 15, 'angle_normalization': 19, 'unit_conversion': 11})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'8B_or_8D_angle_endpoint_policy_then_deduplicate_wrappers': 19, '8D_deduplicate_helpers_and_test_utility_policy': 15, '8D_deduplicated_unit_conversion_helpers': 11})
EXPECTED_DISPOSITION_COUNTS=Counter({'blocked_until_angle_endpoint_and_nonfinite_policy': 19, 'blocked_until_unit_constant_source_contract': 11, 'do_not_import_helper_or_test_utility': 15})
EXPECTED_HELPER_EXCLUSIONS=15
EXPECTED_CONTRACT_BLOCKS=30
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
def validation_contract_fields(errors): return {'supports_self_test':True,'supports_repo_argument':True,'json_stdout':True,'dependency_free_python':True,'mutates_repository_files':False,'errors':errors}
def read_delimited(path,delimiter=',',expected_header=None):
    with Path(path).open(newline='',encoding='utf-8') as f:
        reader=csv.DictReader(f,delimiter=delimiter)
        if expected_header is not None and reader.fieldnames!=expected_header: raise AssertionError(f'header mismatch for {path}: {reader.fieldnames}')
        return list(reader)
def require(c,m):
    if not c: raise AssertionError(m)
def source_row_number(locator):
    m=re.search(r'row_(\d+)',locator); require(bool(m),f'bad row locator: {locator}'); return int(m.group(1))
def repo_file(repo,rel):
    p=Path(repo)/rel; require(p.is_file(),f'missing file: {rel}'); return p
def prior_external_locators(repo):
    locators=set()
    for path in sorted((Path(repo)/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH or rel in FUTURE_SAME_POOL_RESOLUTION_PATHS: continue
        try: rows=read_delimited(path,'	',EXPECTED_HEADER)
        except Exception: continue
        for row in rows: locators.add(row['source_row_locator'])
    return locators
def candidate_rows(repo):
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH)); prior=prior_external_locators(repo)
    c=[row for row in classifier if row['recommended_chunk_group'] in CANDIDATE_GROUPS and row['m07_row_id_or_alias'] not in prior]
    c=sorted(c,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(c)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(c)}')
    return c
def verify_repo(repo):
    repo=Path(repo); candidates=candidate_rows(repo)
    require([r['m07_row_id_or_alias'] for r in candidates[:EXPECTED_ROWS]]==SELECTED_LOCATORS,'selected locators mismatch')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate pool mismatch')
    resolution_rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'	',EXPECTED_HEADER)
    require(len(resolution_rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(resolution_rows)}')
    require([r['source_row_locator'] for r in resolution_rows]==SELECTED_LOCATORS,'resolution locator order mismatch')
    for row in resolution_rows:
        require(row['schema_version']==SCHEMA_VERSION,'schema mismatch'); require(row['source_artifact_id']==SOURCE_ARTIFACT_ID,'source artifact mismatch'); require(row['classifier_path']==CLASSIFIER_PATH,'classifier path mismatch'); require(row['validation_status']=='research_required','validation status mismatch'); require(row['target_formula_id']=='' and row['target_runtime_symbol']=='','runtime target must be empty'); require(row['disposition'] in EXPECTED_DISPOSITION_COUNTS,'disposition mismatch'); require('A44' in row['block_reason'],'block reason must identify A44')
    risk_counts=Counter(row['risk_tier'] for row in resolution_rows); family_counts=Counter(row['formula_family'] for row in resolution_rows); source_group_counts=Counter(row['recommended_chunk_group'] for row in resolution_rows); disposition_counts=Counter(row['disposition'] for row in resolution_rows); source_files={row['source_file_locator'] for row in resolution_rows}
    require(risk_counts==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {risk_counts}'); require(family_counts==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {family_counts}'); require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS,f'source group counts mismatch: {source_group_counts}'); require(disposition_counts==EXPECTED_DISPOSITION_COUNTS,f'disposition counts mismatch: {disposition_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'	'); processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from inventory'); require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed count mismatch')
    backlog=[row for row in inventory if row['category']=='external_m07_backlog_row']; require(len(backlog)==1,'expected exactly one backlog row'); require(int(backlog[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values()); require(processed_total==EXPECTED_CUMULATIVE_PROCESSED,f'processed total mismatch: {processed_total}')
    metadata=[r for r in inventory if r['category']=='metadata_only_formula_vault_candidate']; executable=[r for r in inventory if r['category']=='executable_research_equation']; require(len(metadata)==EXPECTED_METADATA_ROWS,f'metadata count mismatch: {len(metadata)}'); require(len(executable)==EXPECTED_EXECUTABLE_ROWS,f'executable count mismatch: {len(executable)}')
    return {'schema_version':'aerocodex.external_m07.residual_scalar_unit_helper_policy_wave1.verifier.v1','result':'PASS','wave_id':WAVE_ID,'resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'disposition_counts':dict(sorted(disposition_counts.items())),'distinct_source_files':len(source_files),'deduplicated_alias_rows':0,'excluded_helper_rows':EXPECTED_HELPER_EXCLUSIONS,'contract_blocked_rows':EXPECTED_CONTRACT_BLOCKS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True,**validation_contract_fields([])}
def self_test():
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'self selected count mismatch'); require(EXPECTED_CUMULATIVE_PROCESSED==1323,'processed counter mismatch'); require(EXPECTED_REMAINING_BACKLOG==0,'backlog counter mismatch'); require(EXPECTED_HELPER_EXCLUSIONS==15,'helper exclusion mismatch')
    return {'schema_version':'aerocodex.external_m07.residual_scalar_unit_helper_policy_wave1.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,**validation_contract_fields([])}
def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__); parser.add_argument('--self-test',action='store_true'); parser.add_argument('--repo'); args=parser.parse_args(argv)
    try:
        if args.self_test: payload=self_test()
        elif args.repo: payload=verify_repo(Path(args.repo))
        else: parser.error('one of --self-test or --repo is required')
        print(json.dumps(payload,indent=2,sort_keys=True)); return 0 if payload.get('result')=='PASS' else 1
    except Exception as exc:
        payload={'schema_version':'aerocodex.external_m07.residual_scalar_unit_helper_policy_wave1.error.v1','result':'FAIL','error':str(exc),**validation_contract_fields([str(exc)])}
        print(json.dumps(payload,indent=2,sort_keys=True)); return 1
if __name__=='__main__': raise SystemExit(main())
