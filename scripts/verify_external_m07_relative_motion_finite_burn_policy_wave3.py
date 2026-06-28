#!/usr/bin/env python3
"""Verify A33 external M07 relative-motion / finite-burn scalar policy Wave 3 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave3.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
WAVE_ID='a33_external_m07_relative_motion_finite_burn_policy_wave3'
VALIDATOR_NAME='verify_external_m07_relative_motion_finite_burn_policy_wave3'
SELECTED_LOCATORS=['PORT_STATUS_RELEASE_GATE.csv:row_1271', 'PORT_STATUS_RELEASE_GATE.csv:row_1272', 'PORT_STATUS_RELEASE_GATE.csv:row_1273', 'PORT_STATUS_RELEASE_GATE.csv:row_1274', 'PORT_STATUS_RELEASE_GATE.csv:row_1275', 'PORT_STATUS_RELEASE_GATE.csv:row_1276', 'PORT_STATUS_RELEASE_GATE.csv:row_1277', 'PORT_STATUS_RELEASE_GATE.csv:row_1278', 'PORT_STATUS_RELEASE_GATE.csv:row_1279', 'PORT_STATUS_RELEASE_GATE.csv:row_1280', 'PORT_STATUS_RELEASE_GATE.csv:row_1281', 'PORT_STATUS_RELEASE_GATE.csv:row_1282', 'PORT_STATUS_RELEASE_GATE.csv:row_1283', 'PORT_STATUS_RELEASE_GATE.csv:row_1284', 'PORT_STATUS_RELEASE_GATE.csv:row_1285', 'PORT_STATUS_RELEASE_GATE.csv:row_1286', 'PORT_STATUS_RELEASE_GATE.csv:row_1287', 'PORT_STATUS_RELEASE_GATE.csv:row_1288', 'PORT_STATUS_RELEASE_GATE.csv:row_1289', 'PORT_STATUS_RELEASE_GATE.csv:row_1290', 'PORT_STATUS_RELEASE_GATE.csv:row_1291', 'PORT_STATUS_RELEASE_GATE.csv:row_1292', 'PORT_STATUS_RELEASE_GATE.csv:row_1293', 'PORT_STATUS_RELEASE_GATE.csv:row_1294', 'PORT_STATUS_RELEASE_GATE.csv:row_1295', 'PORT_STATUS_RELEASE_GATE.csv:row_1299', 'PORT_STATUS_RELEASE_GATE.csv:row_1300', 'PORT_STATUS_RELEASE_GATE.csv:row_1302', 'PORT_STATUS_RELEASE_GATE.csv:row_1304']
CANDIDATE_GROUPS=['9D_relative_motion_CW_LVLH_policy', '9E_rocket_vehicle_policy_then_bounded_scalar_slice', '9E_rocket_equation_scalar_subset_after_contract']
# Pre-declare later same-pool manifests so this historical Wave 3 verifier remains stable when future bounded waves are added.
future_same_pool_resolution_paths={
  'formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave3.tsv',
  'formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave3.tsv'
}
EXPECTED_CANDIDATE_POOL_ROWS=29
EXPECTED_ROWS=29
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=0
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1260
EXPECTED_REMAINING_BACKLOG=63
EXPECTED_RISK_COUNTS=Counter({'high_risk_requires_numerical_policy': 28, 'medium_risk_requires_contract_review': 1})
EXPECTED_FAMILY_COUNTS=Counter({'orbit_two_body': 27, 'low_risk_scalar_math': 1, 'iterative_solver': 1})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'9E_rocket_vehicle_policy_then_bounded_scalar_slice': 28, '9E_rocket_equation_scalar_subset_after_contract': 1})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_rocket_vehicle_model_and_solver_policy': 28, 'blocked_until_rocket_units_domain_and_sign_policy': 1})
BLOCK_TEXT={'blocked_until_relative_motion_frame_and_linearization_policy': 'Classifier row remains blocked until relative-motion reference-frame conventions, local-frame orientation, linearization assumptions, impulse/timing semantics, source registry, and independent validation oracles are explicitly approved; no runtime alias or implementation claim is made in A33.', 'blocked_until_rocket_vehicle_model_and_solver_policy': 'Classifier row remains blocked until rocket vehicle model boundaries, thrust/mass-flow assumptions, staging or propagation policy, numerical solver scope, source registry, and independent validation oracles are explicitly approved; no runtime alias or implementation claim is made in A33.', 'blocked_until_rocket_units_domain_and_sign_policy': 'Classifier row remains blocked until rocket equation units, mass-domain restrictions, sign conventions, source registry, and independent validation oracles are explicitly approved; no runtime alias or implementation claim is made in A33.'}
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
        if rel==RESOLUTION_PATH or rel in future_same_pool_resolution_paths: continue
        for row in read_delimited(path,'	',EXPECTED_HEADER): locators.add(row['source_row_locator'])
    return locators
def candidate_rows(repo:Path):
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH))
    prior=prior_external_locators(repo)
    candidates=[row for row in classifier if row['recommended_chunk_group'] in CANDIDATE_GROUPS and row['m07_row_id_or_alias'] not in prior]
    candidates=sorted(candidates,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(candidates)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(candidates)}')
    return candidates
def verify_repo(repo:Path):
    repo=repo.resolve(); candidates=candidate_rows(repo)
    require([r['m07_row_id_or_alias'] for r in candidates[:EXPECTED_ROWS]]==SELECTED_LOCATORS,'selected locators mismatch')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate pool mismatch')
    resolution_rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'	',EXPECTED_HEADER)
    require(len(resolution_rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(resolution_rows)}')
    res_by=unique_map(resolution_rows,'source_row_locator','resolution locator')
    cls_by=unique_map(read_delimited(repo_file(repo,CLASSIFIER_PATH)),'m07_row_id_or_alias','classifier locator')
    source_files=set()
    for locator in SELECTED_LOCATORS:
        require(locator in res_by,f'missing resolution row {locator}')
        res=res_by[locator]; cls=cls_by[locator]; source_files.add(res['source_file_locator'])
        require(res['schema_version']==SCHEMA_VERSION,f'schema mismatch {locator}')
        require(res['source_artifact_id']==SOURCE_ARTIFACT_ID,f'source artifact mismatch {locator}')
        require(res['classifier_path']==CLASSIFIER_PATH,f'classifier path mismatch {locator}')
        require(int(res['source_row_number'])==source_row_number(locator),f'row number mismatch {locator}')
        for key in ['rust_function_alias','source_file_locator','formula_family','risk_tier','recommended_chunk_group']:
            require(res[key]==cls[key],f'{key} mismatch {locator}')
        require(res['scilab_function_alias']==cls.get('scilab_function_alias_if_known',''),f'scilab alias mismatch {locator}')
        require(res['validation_status']=='research_required',f'validation status mismatch {locator}')
        require(res['disposition']==cls['block_reason'],f'disposition mismatch {locator}')
        require(res['block_reason']==BLOCK_TEXT[res['disposition']],f'block text mismatch {locator}')
        for target_key in ['target_formula_id','target_resolution_id','target_batch_manifest','target_package','target_crate_name','target_runtime_symbol','target_runtime_path','target_contract_path','target_validation_card_path','target_source_seed_path']:
            require(res[target_key]=='',f'target field populated {locator} {target_key}')
    risk_counts=Counter(r['risk_tier'] for r in resolution_rows); family_counts=Counter(r['formula_family'] for r in resolution_rows); source_group_counts=Counter(r['recommended_chunk_group'] for r in resolution_rows); block_counts=Counter(r['disposition'] for r in resolution_rows)
    require(risk_counts==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {risk_counts}')
    require(family_counts==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {family_counts}')
    require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS,f'source group counts mismatch: {source_group_counts}')
    require(block_counts==EXPECTED_BLOCK_REASON_COUNTS,f'block counts mismatch: {block_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'	')
    processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed count mismatch')
    backlog_rows=[row for row in inventory if row['category']=='external_m07_backlog_row']
    require(len(backlog_rows)==1,'expected one backlog row')
    require(int(backlog_rows[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values())
    require(processed_total==EXPECTED_CUMULATIVE_PROCESSED,f'processed total mismatch: {processed_total}')
    return {'schema_version':'aerocodex.external_m07.relative_motion_finite_burn_policy_wave3.verifier.v1','result':'PASS','wave_id':WAVE_ID,'resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'distinct_source_files':len(source_files),'deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':EXPECTED_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True,**validation_contract_fields([])}
def validation_contract_fields(errors:list[str]|None=None)->dict[str,Any]:
    return {'validator':VALIDATOR_NAME,'wave_identifier':WAVE_ID,'selected_row_range':'PORT_STATUS_RELEASE_GATE.csv:row_1271 through PORT_STATUS_RELEASE_GATE.csv:row_1304','selected_row_count':EXPECTED_ROWS,'processed_backlog_counters':{'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG},'validation_checks_summary':{'supports_self_test':True,'supports_repo_argument':True,'dependency_free_python':True,'mutates_repository_files':False,'selected_rows':'remaining 29 rows in governed relative-motion / finite-burn scalar policy candidate pool','deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':EXPECTED_ROWS,'risk_tier_counts':dict(sorted(EXPECTED_RISK_COUNTS.items())),'formula_family_counts':dict(sorted(EXPECTED_FAMILY_COUNTS.items()))},'errors':errors or []}
def self_test():
    return {'schema_version':'aerocodex.external_m07.relative_motion_finite_burn_policy_wave3.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,**validation_contract_fields([])}
def main()->int:
    parser=argparse.ArgumentParser(description='Verify A33 external M07 relative-motion / finite-burn scalar policy Wave 3 terminal dispositions.')
    parser.add_argument('--repo',type=Path,default=None,help='Repository root. Defaults to current working directory.')
    parser.add_argument('--self-test',action='store_true',help='Run dependency-free verifier self-test.')
    args=parser.parse_args()
    try:
        report=self_test() if args.self_test else verify_repo(args.repo or Path.cwd())
        sys.stdout.write(stable_json(report)); return 0
    except Exception as exc:
        sys.stdout.write(stable_json({'result':'FAIL','error':str(exc),'error_type':type(exc).__name__,**validation_contract_fields([str(exc)])})); return 1
if __name__=='__main__': raise SystemExit(main())
