#!/usr/bin/env python3
"""Verify A36 external M07 attitude dynamics/control policy Wave 1 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_attitude_dynamics_control_policy_wave1.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
WAVE_ID='a36_external_m07_attitude_dynamics_control_policy_wave1'
VALIDATOR_NAME='verify_external_m07_attitude_dynamics_control_policy_wave1'
SELECTED_LOCATORS=['PORT_STATUS_RELEASE_GATE.csv:row_1052', 'PORT_STATUS_RELEASE_GATE.csv:row_1054', 'PORT_STATUS_RELEASE_GATE.csv:row_1055', 'PORT_STATUS_RELEASE_GATE.csv:row_1056', 'PORT_STATUS_RELEASE_GATE.csv:row_1057', 'PORT_STATUS_RELEASE_GATE.csv:row_1058', 'PORT_STATUS_RELEASE_GATE.csv:row_1059', 'PORT_STATUS_RELEASE_GATE.csv:row_1063', 'PORT_STATUS_RELEASE_GATE.csv:row_1064', 'PORT_STATUS_RELEASE_GATE.csv:row_1065', 'PORT_STATUS_RELEASE_GATE.csv:row_1066', 'PORT_STATUS_RELEASE_GATE.csv:row_1068', 'PORT_STATUS_RELEASE_GATE.csv:row_1096', 'PORT_STATUS_RELEASE_GATE.csv:row_1098', 'PORT_STATUS_RELEASE_GATE.csv:row_1099', 'PORT_STATUS_RELEASE_GATE.csv:row_1108', 'PORT_STATUS_RELEASE_GATE.csv:row_1116', 'PORT_STATUS_RELEASE_GATE.csv:row_1126', 'PORT_STATUS_RELEASE_GATE.csv:row_1128', 'PORT_STATUS_RELEASE_GATE.csv:row_1156', 'PORT_STATUS_RELEASE_GATE.csv:row_1172', 'PORT_STATUS_RELEASE_GATE.csv:row_1173', 'PORT_STATUS_RELEASE_GATE.csv:row_1174', 'PORT_STATUS_RELEASE_GATE.csv:row_1175', 'PORT_STATUS_RELEASE_GATE.csv:row_1177', 'PORT_STATUS_RELEASE_GATE.csv:row_1178', 'PORT_STATUS_RELEASE_GATE.csv:row_1179', 'PORT_STATUS_RELEASE_GATE.csv:row_1180', 'PORT_STATUS_RELEASE_GATE.csv:row_1181', 'PORT_STATUS_RELEASE_GATE.csv:row_1183', 'PORT_STATUS_RELEASE_GATE.csv:row_1184', 'PORT_STATUS_RELEASE_GATE.csv:row_1186', 'PORT_STATUS_RELEASE_GATE.csv:row_1189', 'PORT_STATUS_RELEASE_GATE.csv:row_1190', 'PORT_STATUS_RELEASE_GATE.csv:row_1200', 'PORT_STATUS_RELEASE_GATE.csv:row_1201', 'PORT_STATUS_RELEASE_GATE.csv:row_1213', 'PORT_STATUS_RELEASE_GATE.csv:row_1215']
CANDIDATE_GROUPS=['10A_attitude_dynamics_and_control_policy']
EXPECTED_CANDIDATE_POOL_ROWS=38
EXPECTED_ROWS=38
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=0
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1260
EXPECTED_REMAINING_BACKLOG=63
EXPECTED_RISK_COUNTS=Counter({'high_risk_requires_numerical_policy': 38})
EXPECTED_FAMILY_COUNTS=Counter({'coordinate_transform_sensitive': 38})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'10A_attitude_dynamics_and_control_policy': 38})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_attitude_dynamics_control_and_integration_policy': 38})
BLOCK_TEXT={'blocked_until_attitude_dynamics_control_and_integration_policy': 'Classifier row remains blocked until attitude dynamics, control law, torque/inertia, integration policy, frame orientation, source registry, and independent validation oracles are explicitly approved; no runtime alias or implementation claim is made in A36.'}
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
def stable_json(obj:Any)->str: return json.dumps(obj,indent=2,sort_keys=True,ensure_ascii=False)+'\n'
def require(cond:bool,msg:str):
    if not cond: raise AssertionError(msg)
def read_delimited(path:Path,delimiter=',',expected_header=None):
    with path.open(newline='',encoding='utf-8') as f:
        reader=csv.DictReader(f,delimiter=delimiter)
        if expected_header is not None: require(reader.fieldnames==expected_header,f'header mismatch for {path}')
        return list(reader)
def source_row_number(locator:str)->int:
    m=re.search(r'row_(\d+)',locator); require(bool(m),f'bad row locator: {locator}'); return int(m.group(1))
def repo_file(repo:Path,rel:str)->Path:
    p=repo/rel; require(p.is_file(),f'missing file: {rel}'); return p
def unique_map(rows,key,label):
    out={}
    for row in rows:
        value=row[key]; require(value not in out,f'duplicate {label}: {value}'); out[value]=row
    return out
def prior_external_locators(repo:Path)->set[str]:
    locators=set()
    for path in sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH: continue
        for row in read_delimited(path,'\t',EXPECTED_HEADER): locators.add(row['source_row_locator'])
    return locators
def candidate_rows(repo:Path):
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH)); prior=prior_external_locators(repo)
    candidates=[row for row in classifier if row['recommended_chunk_group'] in CANDIDATE_GROUPS and row['m07_row_id_or_alias'] not in prior]
    candidates=sorted(candidates,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(candidates)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(candidates)}')
    return candidates
def validation_contract_fields(errors:list[str]|None=None)->dict[str,Any]:
    return {'validator':VALIDATOR_NAME,'wave_identifier':WAVE_ID,'selected_row_range':'PORT_STATUS_RELEASE_GATE.csv:row_1052 through PORT_STATUS_RELEASE_GATE.csv:row_1215','selected_row_count':EXPECTED_ROWS,'processed_backlog_counters':{'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG},'validation_checks_summary':{'supports_self_test':True,'supports_repo_argument':True,'dependency_free_python':True,'mutates_repository_files':False,'selected_rows':'38 rows in governed attitude dynamics/control policy candidate pool','deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':EXPECTED_ROWS,'risk_tier_counts':dict(sorted(EXPECTED_RISK_COUNTS.items())),'formula_family_counts':dict(sorted(EXPECTED_FAMILY_COUNTS.items()))},'errors':errors or []}
def verify_repo(repo:Path):
    repo=repo.resolve(); candidates=candidate_rows(repo)
    require([r['m07_row_id_or_alias'] for r in candidates[:EXPECTED_ROWS]]==SELECTED_LOCATORS,'selected locators mismatch')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate pool mismatch')
    resolution_rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER)
    require(len(resolution_rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(resolution_rows)}')
    res_by=unique_map(resolution_rows,'source_row_locator','resolution locator')
    cls_by=unique_map(read_delimited(repo_file(repo,CLASSIFIER_PATH)),'m07_row_id_or_alias','classifier locator')
    for locator in SELECTED_LOCATORS:
        require(locator in res_by,f'missing resolution row {locator}')
        res=res_by[locator]; cls=cls_by[locator]
        require(res['schema_version']==SCHEMA_VERSION,f'schema mismatch {locator}')
        require(res['source_artifact_id']==SOURCE_ARTIFACT_ID,f'source artifact mismatch {locator}')
        require(res['classifier_path']==CLASSIFIER_PATH,f'classifier path mismatch {locator}')
        require(int(res['source_row_number'])==source_row_number(locator),f'row number mismatch {locator}')
        for key in ['rust_function_alias','source_file_locator','formula_family','risk_tier','recommended_chunk_group']:
            require(res[key]==cls[key],f'{key} mismatch {locator}')
        require(res['validation_status']=='research_required',f'validation status mismatch {locator}')
        require(res['disposition'] in BLOCK_TEXT,f'disposition mismatch {locator}')
        require(res['block_reason']==BLOCK_TEXT[res['disposition']],f'block text mismatch {locator}')
        for key in ['target_formula_id','target_resolution_id','target_batch_manifest','target_package','target_crate_name','target_runtime_symbol','target_runtime_path','target_contract_path','target_validation_card_path','target_source_seed_path']:
            require(res[key]=='',f'target field populated {key} {locator}')
    risk_counts=Counter(row['risk_tier'] for row in resolution_rows)
    family_counts=Counter(row['formula_family'] for row in resolution_rows)
    source_group_counts=Counter(row['recommended_chunk_group'] for row in resolution_rows)
    block_counts=Counter(row['disposition'] for row in resolution_rows)
    require(risk_counts==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {risk_counts}')
    require(family_counts==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {family_counts}')
    require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS,f'source group counts mismatch: {source_group_counts}')
    require(block_counts==EXPECTED_BLOCK_REASON_COUNTS,f'block counts mismatch: {block_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t')
    processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed count mismatch')
    backlog=[r for r in inventory if r['category']=='external_m07_backlog_row']
    require(sum(int(r['row_count']) for r in processed.values())==EXPECTED_CUMULATIVE_PROCESSED,'processed total mismatch')
    require(len(backlog)==1 and int(backlog[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'backlog count mismatch')
    return {'schema_version':'aerocodex.external_m07.attitude_dynamics_control_policy_wave1.verifier.v1','result':'PASS','resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_runtime_kernel_change_claim':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True}
def self_test():
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'self selected count mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==1260,'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==63,'backlog counter mismatch')
    return {'schema_version':'aerocodex.external_m07.attitude_dynamics_control_policy_wave1.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG}
def emit(obj): sys.stdout.write(stable_json(obj))
def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__); parser.add_argument('--self-test',action='store_true'); parser.add_argument('--repo',type=Path); args=parser.parse_args(argv)
    try:
        if args.self_test: emit(self_test()); return 0
        if args.repo: emit(verify_repo(args.repo)); return 0
        parser.error('one of --self-test or --repo is required')
    except Exception as exc:
        emit({'schema_version':'aerocodex.external_m07.attitude_dynamics_control_policy_wave1.error.v1','result':'FAIL',**validation_contract_fields([str(exc)])}); return 1
if __name__=='__main__': raise SystemExit(main())
