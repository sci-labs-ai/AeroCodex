#!/usr/bin/env python3
"""Verify A31 external M07 relative-motion / finite-burn scalar policy Wave 1 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave1.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
SELECTED_LOCATORS=[
  "PORT_STATUS_RELEASE_GATE.csv:row_0401",
  "PORT_STATUS_RELEASE_GATE.csv:row_0548",
  "PORT_STATUS_RELEASE_GATE.csv:row_0550",
  "PORT_STATUS_RELEASE_GATE.csv:row_0552",
  "PORT_STATUS_RELEASE_GATE.csv:row_0553",
  "PORT_STATUS_RELEASE_GATE.csv:row_0559",
  "PORT_STATUS_RELEASE_GATE.csv:row_0561",
  "PORT_STATUS_RELEASE_GATE.csv:row_0562",
  "PORT_STATUS_RELEASE_GATE.csv:row_0564",
  "PORT_STATUS_RELEASE_GATE.csv:row_0565",
  "PORT_STATUS_RELEASE_GATE.csv:row_0566",
  "PORT_STATUS_RELEASE_GATE.csv:row_0567",
  "PORT_STATUS_RELEASE_GATE.csv:row_0569",
  "PORT_STATUS_RELEASE_GATE.csv:row_0570",
  "PORT_STATUS_RELEASE_GATE.csv:row_0571",
  "PORT_STATUS_RELEASE_GATE.csv:row_0578",
  "PORT_STATUS_RELEASE_GATE.csv:row_0580",
  "PORT_STATUS_RELEASE_GATE.csv:row_0581",
  "PORT_STATUS_RELEASE_GATE.csv:row_0582",
  "PORT_STATUS_RELEASE_GATE.csv:row_0583",
  "PORT_STATUS_RELEASE_GATE.csv:row_0584",
  "PORT_STATUS_RELEASE_GATE.csv:row_0585",
  "PORT_STATUS_RELEASE_GATE.csv:row_0933",
  "PORT_STATUS_RELEASE_GATE.csv:row_0934",
  "PORT_STATUS_RELEASE_GATE.csv:row_0935",
  "PORT_STATUS_RELEASE_GATE.csv:row_0936",
  "PORT_STATUS_RELEASE_GATE.csv:row_0937",
  "PORT_STATUS_RELEASE_GATE.csv:row_0939",
  "PORT_STATUS_RELEASE_GATE.csv:row_0940",
  "PORT_STATUS_RELEASE_GATE.csv:row_0946",
  "PORT_STATUS_RELEASE_GATE.csv:row_0947",
  "PORT_STATUS_RELEASE_GATE.csv:row_0948",
  "PORT_STATUS_RELEASE_GATE.csv:row_0949",
  "PORT_STATUS_RELEASE_GATE.csv:row_0950",
  "PORT_STATUS_RELEASE_GATE.csv:row_0951",
  "PORT_STATUS_RELEASE_GATE.csv:row_0952",
  "PORT_STATUS_RELEASE_GATE.csv:row_0957",
  "PORT_STATUS_RELEASE_GATE.csv:row_0958",
  "PORT_STATUS_RELEASE_GATE.csv:row_0963",
  "PORT_STATUS_RELEASE_GATE.csv:row_0971"
]
CANDIDATE_GROUPS=[
  "9D_relative_motion_CW_LVLH_policy",
  "9E_rocket_vehicle_policy_then_bounded_scalar_slice",
  "9E_rocket_equation_scalar_subset_after_contract"
]
FUTURE_SAME_POOL_RESOLUTION_PATHS={'formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave2.tsv','formula-vault/resolutions/m07_relative_motion_finite_burn_policy_wave3.tsv'}
EXPECTED_CANDIDATE_POOL_ROWS=109
EXPECTED_ROWS=40
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=69
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=786
EXPECTED_REMAINING_BACKLOG=537
EXPECTED_RISK_COUNTS=Counter({'blocked_until_frame_time_policy': 19, 'high_risk_requires_numerical_policy': 19, 'medium_risk_requires_contract_review': 2})
EXPECTED_FAMILY_COUNTS=Counter({'frame_graph_sensitive': 19, 'orbit_two_body': 19, 'low_risk_scalar_math': 2})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'9D_relative_motion_CW_LVLH_policy': 19, '9E_rocket_vehicle_policy_then_bounded_scalar_slice': 19, '9E_rocket_equation_scalar_subset_after_contract': 2})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_relative_motion_frame_and_linearization_policy': 19, 'blocked_until_rocket_vehicle_model_and_solver_policy': 19, 'blocked_until_rocket_units_domain_and_sign_policy': 2})
BLOCK_TEXT={'blocked_until_relative_motion_frame_and_linearization_policy': 'Classifier row remains blocked until relative-motion frame definitions, local-vertical/local-horizontal conventions, linearization domain, reference-orbit assumptions, maneuver targeting contracts, and independent frame-aware validation oracles are explicitly approved; no runtime alias or implementation claim is made in A31.', 'blocked_until_rocket_vehicle_model_and_solver_policy': 'Classifier row remains blocked until rocket vehicle, finite-burn, staging, atmosphere, thrust, mass-flow, and numerical-propagation policies plus independent validation oracles are explicitly approved; no runtime alias or implementation claim is made in A31.', 'blocked_until_rocket_units_domain_and_sign_policy': 'Classifier row remains blocked until rocket-equation unit conventions, mass-ratio domain, delta-v sign policy, thrust-to-weight conventions, source registry, and independent scalar validation oracles are explicitly approved; no runtime alias or implementation claim is made in A31.'}
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
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t')
    processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed count mismatch')
    backlog_rows=[row for row in inventory if row['category']=='external_m07_backlog_row']
    require(len(backlog_rows)==1,'expected one backlog row')
    require(int(backlog_rows[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values())
    require(processed_total==EXPECTED_CUMULATIVE_PROCESSED,f'processed total mismatch: {processed_total}')
    return {'schema_version':'aerocodex.external_m07.relative_motion_finite_burn_policy_wave1.verifier.v1','result':'PASS','resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_runtime_kernel_change_claim':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True}
def self_test():
    require(source_row_number('PORT_STATUS_RELEASE_GATE.csv:row_0401')==401,'row parser failed')
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'selected count mismatch')
    require(SELECTED_LOCATORS[0]=='PORT_STATUS_RELEASE_GATE.csv:row_0401','first locator mismatch')
    require(SELECTED_LOCATORS[-1]=='PORT_STATUS_RELEASE_GATE.csv:row_0971','last locator mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==786,'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==537,'backlog counter mismatch')
    return {'schema_version':'aerocodex.external_m07.relative_motion_finite_burn_policy_wave1.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG}
def main()->int:
    parser=argparse.ArgumentParser(description='Verify A31 external M07 relative-motion / finite-burn scalar policy Wave 1 metadata.')
    parser.add_argument('--repo',type=Path,help='Repository root to validate.')
    parser.add_argument('--self-test',action='store_true',help='Run dependency-free verifier self-test.')
    args=parser.parse_args()
    try:
        report=self_test() if args.self_test else verify_repo(args.repo or Path.cwd())
        sys.stdout.write(stable_json(report)); return 0
    except Exception as exc:
        sys.stdout.write(stable_json({'result':'FAIL','error':str(exc),'error_type':type(exc).__name__})); return 1
if __name__=='__main__': raise SystemExit(main())
