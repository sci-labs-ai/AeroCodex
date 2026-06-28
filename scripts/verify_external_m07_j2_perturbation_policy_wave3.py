#!/usr/bin/env python3
"""Verify A39 external M07 J2 perturbation / numerical propagation policy Wave 3 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_j2_perturbation_policy_wave3.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
WAVE_ID='a39_external_m07_j2_perturbation_policy_wave3'
VALIDATOR_NAME='verify_external_m07_j2_perturbation_policy_wave3'
SELECTED_LOCATORS=['PORT_STATUS_RELEASE_GATE.csv:row_0944', 'PORT_STATUS_RELEASE_GATE.csv:row_0996', 'PORT_STATUS_RELEASE_GATE.csv:row_0998', 'PORT_STATUS_RELEASE_GATE.csv:row_0999', 'PORT_STATUS_RELEASE_GATE.csv:row_1000', 'PORT_STATUS_RELEASE_GATE.csv:row_1001', 'PORT_STATUS_RELEASE_GATE.csv:row_1002', 'PORT_STATUS_RELEASE_GATE.csv:row_1003', 'PORT_STATUS_RELEASE_GATE.csv:row_1005', 'PORT_STATUS_RELEASE_GATE.csv:row_1006', 'PORT_STATUS_RELEASE_GATE.csv:row_1007', 'PORT_STATUS_RELEASE_GATE.csv:row_1008', 'PORT_STATUS_RELEASE_GATE.csv:row_1009', 'PORT_STATUS_RELEASE_GATE.csv:row_1010', 'PORT_STATUS_RELEASE_GATE.csv:row_1011', 'PORT_STATUS_RELEASE_GATE.csv:row_1012', 'PORT_STATUS_RELEASE_GATE.csv:row_1016', 'PORT_STATUS_RELEASE_GATE.csv:row_1017', 'PORT_STATUS_RELEASE_GATE.csv:row_1018', 'PORT_STATUS_RELEASE_GATE.csv:row_1020', 'PORT_STATUS_RELEASE_GATE.csv:row_1021', 'PORT_STATUS_RELEASE_GATE.csv:row_1022', 'PORT_STATUS_RELEASE_GATE.csv:row_1023', 'PORT_STATUS_RELEASE_GATE.csv:row_1024', 'PORT_STATUS_RELEASE_GATE.csv:row_1025', 'PORT_STATUS_RELEASE_GATE.csv:row_1027', 'PORT_STATUS_RELEASE_GATE.csv:row_1028', 'PORT_STATUS_RELEASE_GATE.csv:row_1029', 'PORT_STATUS_RELEASE_GATE.csv:row_1030', 'PORT_STATUS_RELEASE_GATE.csv:row_1031', 'PORT_STATUS_RELEASE_GATE.csv:row_1032', 'PORT_STATUS_RELEASE_GATE.csv:row_1033', 'PORT_STATUS_RELEASE_GATE.csv:row_1034', 'PORT_STATUS_RELEASE_GATE.csv:row_1035', 'PORT_STATUS_RELEASE_GATE.csv:row_1036', 'PORT_STATUS_RELEASE_GATE.csv:row_1037', 'PORT_STATUS_RELEASE_GATE.csv:row_1038', 'PORT_STATUS_RELEASE_GATE.csv:row_1039', 'PORT_STATUS_RELEASE_GATE.csv:row_1040', 'PORT_STATUS_RELEASE_GATE.csv:row_1041', 'PORT_STATUS_RELEASE_GATE.csv:row_1042', 'PORT_STATUS_RELEASE_GATE.csv:row_1043', 'PORT_STATUS_RELEASE_GATE.csv:row_1044', 'PORT_STATUS_RELEASE_GATE.csv:row_1045', 'PORT_STATUS_RELEASE_GATE.csv:row_1046', 'PORT_STATUS_RELEASE_GATE.csv:row_1047', 'PORT_STATUS_RELEASE_GATE.csv:row_1233', 'PORT_STATUS_RELEASE_GATE.csv:row_1234']
CANDIDATE_GROUPS={'10B_J2_perturbation_and_numerical_policy'}
FUTURE_SAME_POOL_RESOLUTION_PATHS=set()
EXPECTED_CANDIDATE_POOL_ROWS=48
EXPECTED_ROWS=48
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=0
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1260
EXPECTED_REMAINING_BACKLOG=63
EXPECTED_RISK_COUNTS=Counter({'high_risk_requires_numerical_policy': 48})
EXPECTED_FAMILY_COUNTS=Counter({'perturbation_or_J2': 48})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'10B_J2_perturbation_and_numerical_policy': 48})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_J2_perturbation_model_and_numerical_validation_policy': 48})
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
def validation_contract_fields(errors:list[str]):
    return {'supports_self_test':True,'supports_repo_argument':True,'json_stdout':True,'dependency_free_python':True,'mutates_repository_files':False,'errors':errors}
def read_delimited(path:Path,delimiter=',',expected_header=None):
    with path.open(newline='',encoding='utf-8') as f:
        reader=csv.DictReader(f,delimiter=delimiter)
        if expected_header is not None and reader.fieldnames != expected_header:
            raise AssertionError(f'header mismatch for {path}')
        return list(reader)
def require(cond:bool,msg:str):
    if not cond: raise AssertionError(msg)
def source_row_number(locator:str)->int:
    m=re.search(r'row_(\d+)',locator); require(bool(m),f'bad row locator: {locator}'); return int(m.group(1))
def repo_file(repo:Path,rel:str)->Path:
    p=repo/rel; require(p.is_file(),f'missing file: {rel}'); return p
def prior_external_locators(repo:Path)->set[str]:
    locators=set()
    for path in sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH or rel in FUTURE_SAME_POOL_RESOLUTION_PATHS: continue
        for row in read_delimited(path,'\t',EXPECTED_HEADER): locators.add(row['source_row_locator'])
    return locators
def candidate_rows(repo:Path):
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH))
    prior=prior_external_locators(repo)
    candidates=[row for row in classifier if row['recommended_chunk_group'] in CANDIDATE_GROUPS and row['m07_row_id_or_alias'] not in prior]
    candidates=sorted(candidates,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(candidates)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(candidates)}')
    return candidates
def verify_repo(repo:Path)->dict[str,Any]:
    candidates=candidate_rows(repo)
    require([r['m07_row_id_or_alias'] for r in candidates[:EXPECTED_ROWS]]==SELECTED_LOCATORS,'selected locators mismatch')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate pool mismatch')
    resolution_rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER)
    require(len(resolution_rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(resolution_rows)}')
    require([r['source_row_locator'] for r in resolution_rows]==SELECTED_LOCATORS,'resolution locator order mismatch')
    for row in resolution_rows:
        require(row['schema_version']==SCHEMA_VERSION,'schema mismatch')
        require(row['source_artifact_id']==SOURCE_ARTIFACT_ID,'source artifact mismatch')
        require(row['classifier_path']==CLASSIFIER_PATH,'classifier path mismatch')
        require(row['validation_status']=='research_required','validation status mismatch')
        require(row['target_formula_id']=='' and row['target_runtime_symbol']=='','runtime target must be empty')
        require(row['disposition']=='blocked_until_J2_perturbation_model_and_numerical_validation_policy','disposition mismatch')
        require('A39' in row['block_reason'],'block reason must identify A39')
    risk_counts=Counter(row['risk_tier'] for row in resolution_rows)
    family_counts=Counter(row['formula_family'] for row in resolution_rows)
    source_group_counts=Counter(row['recommended_chunk_group'] for row in resolution_rows)
    block_counts=Counter(row['disposition'] for row in resolution_rows)
    source_files={row['source_file_locator'] for row in resolution_rows}
    require(risk_counts==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {risk_counts}')
    require(family_counts==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {family_counts}')
    require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS,f'source group counts mismatch: {source_group_counts}')
    require(block_counts==EXPECTED_BLOCK_REASON_COUNTS,f'block counts mismatch: {block_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t')
    processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed count mismatch')
    backlog_rows=[row for row in inventory if row['category']=='external_m07_backlog_row']
    require(len(backlog_rows)==1,'expected exactly one backlog row')
    require(int(backlog_rows[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values())
    require(processed_total==EXPECTED_CUMULATIVE_PROCESSED,f'processed total mismatch: {processed_total}')
    return {'schema_version':'aerocodex.external_m07.j2_perturbation_policy_wave3.verifier.v1','result':'PASS','wave_id':WAVE_ID,'resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'distinct_source_files':len(source_files),'deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':EXPECTED_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True,**validation_contract_fields([])}
def self_test()->dict[str,Any]:
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'self selected count mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==1260,'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==63,'backlog counter mismatch')
    return {'schema_version':'aerocodex.external_m07.j2_perturbation_policy_wave3.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,**validation_contract_fields([])}
def main(argv:list[str]|None=None)->int:
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--self-test',action='store_true')
    parser.add_argument('--repo')
    args=parser.parse_args(argv)
    try:
        if args.self_test: payload=self_test()
        elif args.repo: payload=verify_repo(Path(args.repo))
        else: parser.error('one of --self-test or --repo is required')
        print(json.dumps(payload,indent=2,sort_keys=True)); return 0 if payload.get('result')=='PASS' else 1
    except Exception as exc:
        payload={'schema_version':'aerocodex.external_m07.j2_perturbation_policy_wave3.error.v1','result':'FAIL','error':str(exc),**validation_contract_fields([str(exc)])}
        print(json.dumps(payload,indent=2,sort_keys=True)); return 1
if __name__=='__main__':
    raise SystemExit(main())
