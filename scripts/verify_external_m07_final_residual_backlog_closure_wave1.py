#!/usr/bin/env python3
"""Verify A45 external M07 final residual backlog closure Wave 1 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_final_residual_backlog_closure_wave1.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
WAVE_ID='a45_external_m07_final_residual_backlog_closure_wave1'
SELECTED_LOCATORS=['PORT_STATUS_RELEASE_GATE.csv:row_0807', 'PORT_STATUS_RELEASE_GATE.csv:row_0808', 'PORT_STATUS_RELEASE_GATE.csv:row_1140', 'PORT_STATUS_RELEASE_GATE.csv:row_1220', 'PORT_STATUS_RELEASE_GATE.csv:row_1221', 'PORT_STATUS_RELEASE_GATE.csv:row_1222', 'PORT_STATUS_RELEASE_GATE.csv:row_1223', 'PORT_STATUS_RELEASE_GATE.csv:row_1224', 'PORT_STATUS_RELEASE_GATE.csv:row_1225', 'PORT_STATUS_RELEASE_GATE.csv:row_1226', 'PORT_STATUS_RELEASE_GATE.csv:row_1227', 'PORT_STATUS_RELEASE_GATE.csv:row_1228', 'PORT_STATUS_RELEASE_GATE.csv:row_1229', 'PORT_STATUS_RELEASE_GATE.csv:row_1230', 'PORT_STATUS_RELEASE_GATE.csv:row_1231', 'PORT_STATUS_RELEASE_GATE.csv:row_1232', 'PORT_STATUS_RELEASE_GATE.csv:row_1296', 'PORT_STATUS_RELEASE_GATE.csv:row_1297']
CANDIDATE_GROUPS={'10E_classifier_refresh_or_manual_source_review', '8D_deduplicate_helpers_and_test_utility_policy'}
EXPECTED_CANDIDATE_POOL_ROWS=18
EXPECTED_ROWS=18
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=0
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1323
EXPECTED_REMAINING_BACKLOG=0
EXPECTED_RISK_COUNTS=Counter({'do_not_import': 5, 'medium_risk_requires_contract_review': 13})
EXPECTED_FAMILY_COUNTS=Counter({'ambiguous_source_or_contract': 18})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'8D_deduplicate_helpers_and_test_utility_policy': 5, '10E_classifier_refresh_or_manual_source_review': 13})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_test_helper_or_duplicate_utility_not_formula': 5, 'blocked_until_family_contract_source_review_and_validation_evidence': 13})

def read_delimited(path, delimiter):
    with path.open(newline='', encoding='utf-8') as handle:
        return list(csv.DictReader(handle, delimiter=delimiter))

def repo_file(repo, rel):
    path=repo / rel
    if not path.is_file():
        raise AssertionError(f'missing required file: {rel}')
    return path

def require(condition, message):
    if not condition:
        raise AssertionError(message)

def validation_contract_fields(errors):
    return {'supports_self_test':True,'supports_repo_argument':True,'dependency_free_python':True,'json_stdout':True,'mutates_repository_files':False,'errors':errors}

def row_num_from_locator(locator):
    m=re.search(r'row_(\d+)', locator or '')
    require(bool(m), f'bad row locator: {locator}')
    return int(m.group(1))

def prior_external_locators(repo):
    locators=set()
    for path in sorted((Path(repo)/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH:
            continue
        for row in read_delimited(path, '	'):
            if row.get('source_row_locator'):
                locators.add(row['source_row_locator'])
    return locators

def classify_candidates(repo):
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH), ',')
    prior=prior_external_locators(repo)
    rows=[]
    for row in classifier:
        if row.get('recommended_chunk_group') in CANDIDATE_GROUPS and row.get('m07_row_id_or_alias') not in prior:
            rows.append(row)
    rows.sort(key=lambda r: row_num_from_locator(r['m07_row_id_or_alias']))
    return rows

def verify_repo(repo):
    repo=Path(repo)
    candidates=classify_candidates(repo)
    candidate_locators=[row['m07_row_id_or_alias'] for row in candidates]
    require(len(candidates)==EXPECTED_CANDIDATE_POOL_ROWS, f'candidate pool count mismatch: {len(candidates)}')
    require(candidate_locators[:EXPECTED_ROWS]==SELECTED_LOCATORS, 'selected row locators do not match candidate-pool prefix')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS, 'remaining candidate pool mismatch')
    resolution=read_delimited(repo_file(repo,RESOLUTION_PATH), '	')
    require(len(resolution)==EXPECTED_ROWS, f'resolution row count mismatch: {len(resolution)}')
    require([row['source_row_locator'] for row in resolution]==SELECTED_LOCATORS, 'resolution source locators mismatch')
    require(all(row['schema_version']==SCHEMA_VERSION for row in resolution), 'resolution schema mismatch')
    require(all(row['source_artifact_id']==SOURCE_ARTIFACT_ID for row in resolution), 'source artifact mismatch')
    require(all(row['validation_status']=='research_required' for row in resolution), 'validation status mismatch')
    require(all(row['target_formula_id']=='' and row['target_runtime_symbol']=='' for row in resolution), 'runtime target fields must be empty')
    source_group_counts=Counter(row['recommended_chunk_group'] for row in resolution)
    risk_counts=Counter(row['risk_tier'] for row in resolution)
    family_counts=Counter(row['formula_family'] for row in resolution)
    block_counts=Counter(row['disposition'] for row in resolution)
    source_files={row['source_file_locator'] for row in resolution}
    require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS, f'source group counts mismatch: {source_group_counts}')
    require(risk_counts==EXPECTED_RISK_COUNTS, f'risk counts mismatch: {risk_counts}')
    require(family_counts==EXPECTED_FAMILY_COUNTS, f'family counts mismatch: {family_counts}')
    require(block_counts==EXPECTED_BLOCK_REASON_COUNTS, f'block counts mismatch: {block_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH), '	')
    processed={row['source_path']: row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed, 'new resolution missing from inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS, 'new processed row count mismatch')
    backlog=[row for row in inventory if row['category']=='external_m07_backlog_row']
    require(len(backlog)==1, 'expected exactly one backlog row')
    require(int(backlog[0]['row_count'])==EXPECTED_REMAINING_BACKLOG, 'backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values())
    require(processed_total==EXPECTED_CUMULATIVE_PROCESSED, f'processed total mismatch: {processed_total}')
    metadata=[r for r in inventory if r['category']=='metadata_only_formula_vault_candidate']
    executable=[r for r in inventory if r['category']=='executable_research_equation']
    require(len(metadata)==EXPECTED_METADATA_ROWS, f'metadata count mismatch: {len(metadata)}')
    require(len(executable)==EXPECTED_EXECUTABLE_ROWS, f'executable count mismatch: {len(executable)}')
    excluded_helper_rows=sum(1 for row in resolution if row['risk_tier']=='do_not_import' or row['disposition']=='blocked_test_helper_or_duplicate_utility_not_formula')
    contract_blocked_rows=EXPECTED_ROWS-excluded_helper_rows
    return {'schema_version':'aerocodex.external_m07.final_residual_backlog_closure_wave1.verifier.v1','result':'PASS','wave_id':WAVE_ID,'resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'distinct_source_files':len(source_files),'deduplicated_alias_rows':0,'excluded_helper_rows':excluded_helper_rows,'contract_blocked_rows':contract_blocked_rows,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True,**validation_contract_fields([])}

def self_test():
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS, 'self selected count mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==1323, 'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==0, 'backlog counter mismatch')
    require(EXPECTED_REMAINING_CANDIDATE_POOL_ROWS==0, 'remaining candidate pool mismatch')
    return {'schema_version':'aerocodex.external_m07.final_residual_backlog_closure_wave1.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,**validation_contract_fields([])}

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--repo', type=Path)
    parser.add_argument('--self-test', action='store_true')
    args=parser.parse_args()
    try:
        if args.self_test:
            result=self_test()
        else:
            result=verify_repo(args.repo or Path.cwd())
    except Exception as exc:
        print(json.dumps({'schema_version':'aerocodex.external_m07.final_residual_backlog_closure_wave1.error.v1','result':'FAIL','error':str(exc),**validation_contract_fields([str(exc)])}, indent=2, sort_keys=True))
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0
if __name__=='__main__':
    sys.exit(main())
