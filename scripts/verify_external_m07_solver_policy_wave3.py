#!/usr/bin/env python3
"""Verify A30 external M07 solver / numerical propagation policy Wave 3 terminal dispositions."""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_solver_policy_wave3.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
SELECTED_LOCATORS=[
  "PORT_STATUS_RELEASE_GATE.csv:row_0885",
  "PORT_STATUS_RELEASE_GATE.csv:row_0886",
  "PORT_STATUS_RELEASE_GATE.csv:row_0887",
  "PORT_STATUS_RELEASE_GATE.csv:row_0888",
  "PORT_STATUS_RELEASE_GATE.csv:row_0893",
  "PORT_STATUS_RELEASE_GATE.csv:row_0894",
  "PORT_STATUS_RELEASE_GATE.csv:row_0905",
  "PORT_STATUS_RELEASE_GATE.csv:row_0906",
  "PORT_STATUS_RELEASE_GATE.csv:row_0909",
  "PORT_STATUS_RELEASE_GATE.csv:row_0910",
  "PORT_STATUS_RELEASE_GATE.csv:row_0921",
  "PORT_STATUS_RELEASE_GATE.csv:row_0922",
  "PORT_STATUS_RELEASE_GATE.csv:row_0924",
  "PORT_STATUS_RELEASE_GATE.csv:row_0927",
  "PORT_STATUS_RELEASE_GATE.csv:row_0928",
  "PORT_STATUS_RELEASE_GATE.csv:row_0929",
  "PORT_STATUS_RELEASE_GATE.csv:row_0943",
  "PORT_STATUS_RELEASE_GATE.csv:row_0961",
  "PORT_STATUS_RELEASE_GATE.csv:row_0962",
  "PORT_STATUS_RELEASE_GATE.csv:row_0984",
  "PORT_STATUS_RELEASE_GATE.csv:row_0985",
  "PORT_STATUS_RELEASE_GATE.csv:row_0997",
  "PORT_STATUS_RELEASE_GATE.csv:row_1004",
  "PORT_STATUS_RELEASE_GATE.csv:row_1014",
  "PORT_STATUS_RELEASE_GATE.csv:row_1019",
  "PORT_STATUS_RELEASE_GATE.csv:row_1026",
  "PORT_STATUS_RELEASE_GATE.csv:row_1067",
  "PORT_STATUS_RELEASE_GATE.csv:row_1090",
  "PORT_STATUS_RELEASE_GATE.csv:row_1091",
  "PORT_STATUS_RELEASE_GATE.csv:row_1092",
  "PORT_STATUS_RELEASE_GATE.csv:row_1093",
  "PORT_STATUS_RELEASE_GATE.csv:row_1117",
  "PORT_STATUS_RELEASE_GATE.csv:row_1162",
  "PORT_STATUS_RELEASE_GATE.csv:row_1163",
  "PORT_STATUS_RELEASE_GATE.csv:row_1176",
  "PORT_STATUS_RELEASE_GATE.csv:row_1202",
  "PORT_STATUS_RELEASE_GATE.csv:row_1209",
  "PORT_STATUS_RELEASE_GATE.csv:row_1212",
  "PORT_STATUS_RELEASE_GATE.csv:row_1239",
  "PORT_STATUS_RELEASE_GATE.csv:row_1242",
  "PORT_STATUS_RELEASE_GATE.csv:row_1243",
  "PORT_STATUS_RELEASE_GATE.csv:row_1244",
  "PORT_STATUS_RELEASE_GATE.csv:row_1251"
]
CANDIDATE_GROUPS=['9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy','9C_or_10B_generic_numerical_method_policy','9C_solver_rank_tolerance_and_observation_policy','9C_solver_rank_tolerance_policy_before_any_promotion']
EXPECTED_CANDIDATE_POOL_ROWS=43
EXPECTED_ROWS=43
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=0
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1125
EXPECTED_REMAINING_BACKLOG=198
EXPECTED_RISK_COUNTS=Counter({'blocked_until_solver_policy':43})
EXPECTED_FAMILY_COUNTS=Counter({'iterative_solver':43})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy':43})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_iteration_root_selection_and_equivalence_policy':43})
BLOCK_TEXT={'blocked_until_iteration_root_selection_and_equivalence_policy': 'Classifier row remains blocked until solver iteration limits, root selection, branch handling, convergence failure states, tolerance policy, source registry, and independent equivalence oracles are explicitly approved; no runtime alias or implementation claim is made in A30.'}
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
    return {'schema_version':'aerocodex.external_m07.solver_policy_wave3.verifier.v1','result':'PASS','resolution_path':RESOLUTION_PATH,'selected_rows':SELECTED_LOCATORS,'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':EXPECTED_ROWS,'source_group_counts':dict(sorted(source_group_counts.items())),'risk_tier_counts':dict(sorted(risk_counts.items())),'formula_family_counts':dict(sorted(family_counts.items())),'block_reason_counts':dict(sorted(block_counts.items())),'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,'metadata_inventory_records':EXPECTED_METADATA_ROWS,'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_runtime_kernel_change_claim':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True}
def self_test():
    require(source_row_number('PORT_STATUS_RELEASE_GATE.csv:row_0885')==885,'row parser failed')
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'selected count mismatch')
    require(SELECTED_LOCATORS[0]=='PORT_STATUS_RELEASE_GATE.csv:row_0885','first locator mismatch')
    require(SELECTED_LOCATORS[-1]=='PORT_STATUS_RELEASE_GATE.csv:row_1251','last locator mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==1125,'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==198,'backlog counter mismatch')
    return {'schema_version':'aerocodex.external_m07.solver_policy_wave3.self_test.v1','result':'PASS','selected_count':len(SELECTED_LOCATORS),'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG}
def main()->int:
    parser=argparse.ArgumentParser(description='Verify A30 external M07 solver / numerical propagation policy Wave 3 metadata.')
    parser.add_argument('--repo',type=Path,help='Repository root to validate.')
    parser.add_argument('--self-test',action='store_true',help='Run dependency-free verifier self-test.')
    args=parser.parse_args()
    try:
        report=self_test() if args.self_test else verify_repo(args.repo or Path.cwd())
        sys.stdout.write(stable_json(report)); return 0
    except Exception as exc:
        sys.stdout.write(stable_json({'result':'FAIL','error':str(exc),'error_type':type(exc).__name__})); return 1
if __name__=='__main__': raise SystemExit(main())
