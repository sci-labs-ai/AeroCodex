#!/usr/bin/env python3
"""Verify A26 coordinate-transform / frame-graph / time-scale policy Wave 1 terminal dispositions.

This dependency-free verifier consumes governed classifier metadata and external
resolution manifests. It never opens or parses raw Rust-port, M07, or Scilab
source text.
"""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any,Iterable
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_coordinate_transform_frame_time_policy_wave1.tsv'
future_same_pool_resolution_paths={
  'formula-vault/resolutions/m07_coordinate_transform_frame_time_policy_wave2.tsv'
}
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
SELECTED_LOCATORS=[
  "PORT_STATUS_RELEASE_GATE.csv:row_0013",
  "PORT_STATUS_RELEASE_GATE.csv:row_0123",
  "PORT_STATUS_RELEASE_GATE.csv:row_0124",
  "PORT_STATUS_RELEASE_GATE.csv:row_0125",
  "PORT_STATUS_RELEASE_GATE.csv:row_0126",
  "PORT_STATUS_RELEASE_GATE.csv:row_0127",
  "PORT_STATUS_RELEASE_GATE.csv:row_0128",
  "PORT_STATUS_RELEASE_GATE.csv:row_0129",
  "PORT_STATUS_RELEASE_GATE.csv:row_0130",
  "PORT_STATUS_RELEASE_GATE.csv:row_0131",
  "PORT_STATUS_RELEASE_GATE.csv:row_0132",
  "PORT_STATUS_RELEASE_GATE.csv:row_0133",
  "PORT_STATUS_RELEASE_GATE.csv:row_0134",
  "PORT_STATUS_RELEASE_GATE.csv:row_0135",
  "PORT_STATUS_RELEASE_GATE.csv:row_0136",
  "PORT_STATUS_RELEASE_GATE.csv:row_0137",
  "PORT_STATUS_RELEASE_GATE.csv:row_0138",
  "PORT_STATUS_RELEASE_GATE.csv:row_0139",
  "PORT_STATUS_RELEASE_GATE.csv:row_0140",
  "PORT_STATUS_RELEASE_GATE.csv:row_0141",
  "PORT_STATUS_RELEASE_GATE.csv:row_0142",
  "PORT_STATUS_RELEASE_GATE.csv:row_0143",
  "PORT_STATUS_RELEASE_GATE.csv:row_0144",
  "PORT_STATUS_RELEASE_GATE.csv:row_0145",
  "PORT_STATUS_RELEASE_GATE.csv:row_0146",
  "PORT_STATUS_RELEASE_GATE.csv:row_0147",
  "PORT_STATUS_RELEASE_GATE.csv:row_0156",
  "PORT_STATUS_RELEASE_GATE.csv:row_0167",
  "PORT_STATUS_RELEASE_GATE.csv:row_0168",
  "PORT_STATUS_RELEASE_GATE.csv:row_0169",
  "PORT_STATUS_RELEASE_GATE.csv:row_0170",
  "PORT_STATUS_RELEASE_GATE.csv:row_0173",
  "PORT_STATUS_RELEASE_GATE.csv:row_0185",
  "PORT_STATUS_RELEASE_GATE.csv:row_0187",
  "PORT_STATUS_RELEASE_GATE.csv:row_0239",
  "PORT_STATUS_RELEASE_GATE.csv:row_0241",
  "PORT_STATUS_RELEASE_GATE.csv:row_0324",
  "PORT_STATUS_RELEASE_GATE.csv:row_0332",
  "PORT_STATUS_RELEASE_GATE.csv:row_0339",
  "PORT_STATUS_RELEASE_GATE.csv:row_0404"
]
CANDIDATE_GROUPS=[
  "9B_coordinate_transform_contracts_after_frame_policy",
  "9B_frame_graph_time_policy_before_coordinate_transforms",
  "9B_time_scale_and_sidereal_policy"
]
M07_REPRESENTED_FUNCTION_ROWS=1350
EXPECTED_CANDIDATE_POOL_ROWS=85
EXPECTED_ROWS=40
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=45
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=1125
EXPECTED_REMAINING_BACKLOG=198
EXPECTED_RISK_COUNTS=Counter({'medium_risk_requires_contract_review': 29, 'blocked_until_frame_time_policy': 11})
EXPECTED_FAMILY_COUNTS=Counter({'coordinate_transform_sensitive': 29, 'time_scale_sensitive': 9, 'frame_graph_sensitive': 2})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_coordinate_frame_sign_and_rotation_order_contract': 29, 'blocked_until_epoch_time_scale_and_sidereal_policy': 9, 'blocked_until_frame_graph_and_time_policy': 2})
BLOCK_TEXT={
  "blocked_until_coordinate_frame_sign_and_rotation_order_contract": "Classifier row remains blocked until coordinate-frame sign, axis order, rotation order, round-trip contract, source registry, and independent coordinate-transform oracles are explicitly approved; no runtime alias or implementation claim is made in A26.",
  "blocked_until_epoch_time_scale_and_sidereal_policy": "Classifier row remains blocked until epoch, time-scale, and sidereal-day policy, source registry, and independent time oracles are explicitly approved; no runtime alias or implementation claim is made in A26.",
  "blocked_until_frame_graph_and_time_policy": "Classifier row remains blocked until frame-graph semantics, epoch/time-scale policy, source registry, and independent frame/time oracle fixtures are explicitly approved; no runtime alias or implementation claim is made in A26."
}
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
VALIDATOR_NAME='verify_external_m07_coordinate_transform_frame_time_policy_wave1'
WAVE_ID='a26_external_m07_coordinate_transform_frame_time_policy_wave1'
SELECTED_ROW_RANGE={'first':SELECTED_LOCATORS[0],'last':SELECTED_LOCATORS[-1],'count':EXPECTED_ROWS}
class VerificationError(RuntimeError): pass
def require(c:bool,m:str)->None:
    if not c: raise VerificationError(m)
def stable_json(v:Any)->str:return json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+'\n'
def read_delimited(path:Path,delimiter:str=',',expected_header:list[str]|None=None)->list[dict[str,str]]:
    require(path.is_file(),f'missing file: {path}')
    with path.open(encoding='utf-8-sig',newline='') as f:
        reader=csv.DictReader(f,delimiter=delimiter); require(reader.fieldnames is not None,f'missing header: {path}')
        if expected_header is not None: require(reader.fieldnames==expected_header,f'header mismatch: {path}')
        return list(reader)
def repo_file(repo:Path,relative:str)->Path:
    p=(repo/relative).resolve(); require(repo==p or repo in p.parents,f'path escapes repo: {relative}'); require(p.is_file(),f'missing repository file: {relative}'); return p
def source_row_number(locator:str)->int:
    m=re.fullmatch(r'PORT_STATUS_RELEASE_GATE\.csv:row_(\d+)',locator); require(m is not None,f'invalid source row locator: {locator}'); return int(m.group(1))
def unique_map(rows:Iterable[dict[str,str]],key:str,label:str)->dict[str,dict[str,str]]:
    out={}
    for row in rows:
        value=row[key]; require(value and value not in out,f'duplicate or empty {label} {key}: {value!r}'); out[value]=row
    return out
def external_resolution_inventory(repo:Path,inventory:list[dict[str,str]],metadata_count:int)->tuple[int,int,set[str]]:
    processed=[r for r in inventory if r['category']=='external_m07_processed_row']; backlog=[r for r in inventory if r['category']=='external_m07_backlog_row']; processed_map=unique_map(processed,'source_path','external processed inventory')
    manifests=sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')); require(manifests,'no external M07 resolution manifests found')
    expected_paths=set(); total=0; locators=set(); resolution_ids=set()
    for path in manifests:
        relative=path.relative_to(repo).as_posix(); rows=read_delimited(path,'\t',EXPECTED_HEADER); expected_paths.add(relative); inv=processed_map.get(relative); require(inv is not None,f'missing processed inventory row for {relative}'); require(inv['row_count']==str(len(rows)),f'processed inventory count mismatch for {relative}'); total+=len(rows)
        for row in rows:
            require(row['source_row_locator'] not in locators,f'duplicate external source row locator: {row["source_row_locator"]}'); locators.add(row['source_row_locator']); require(row['resolution_id'] not in resolution_ids,f'duplicate external resolution ID: {row["resolution_id"]}'); resolution_ids.add(row['resolution_id'])
    require(set(processed_map)==expected_paths,'processed inventory and external manifests are not exact union'); require(len(backlog)==1,f'expected one backlog row, found {len(backlog)}'); expected=M07_REPRESENTED_FUNCTION_ROWS-metadata_count-total; require(backlog[0]['row_count']==str(expected),'external backlog count mismatch'); return total,expected,locators
def verify_repo(repo:Path)->dict[str,Any]:
    repo=repo.resolve(); classifier_rows=read_delimited(repo_file(repo,CLASSIFIER_PATH),','); classifier=unique_map(classifier_rows,'m07_row_id_or_alias','classifier')
    prior_locators=set()
    for path in sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH or rel in future_same_pool_resolution_paths: continue
        for row in read_delimited(path,'\t',EXPECTED_HEADER): prior_locators.add(row['source_row_locator'])
    candidate_pool=[]
    for row in classifier_rows:
        if row['recommended_chunk_group'] in CANDIDATE_GROUPS and row['m07_row_id_or_alias'] not in prior_locators:
            candidate_pool.append(row)
    candidate_pool=sorted(candidate_pool,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(candidate_pool)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(candidate_pool)}')
    selected=[classifier[loc] for loc in SELECTED_LOCATORS]
    require([r['m07_row_id_or_alias'] for r in candidate_pool[:EXPECTED_ROWS]]==SELECTED_LOCATORS,'selected locators are not the first bounded source-ordered candidate-pool slice')
    require(len(candidate_pool[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate-pool count mismatch')
    risk=Counter(r['risk_tier'] for r in selected); family=Counter(r['formula_family'] for r in selected); groups=Counter(r['recommended_chunk_group'] for r in selected); block=Counter(r['block_reason'] for r in selected)
    require(risk==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {dict(risk)}'); require(family==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {dict(family)}'); require(block==EXPECTED_BLOCK_REASON_COUNTS,f'block counts mismatch: {dict(block)}')
    rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER); require(len(rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(rows)}'); resolutions=unique_map(rows,'source_row_locator','resolution'); unique_map(rows,'resolution_id','resolution'); require(list(resolutions)==SELECTED_LOCATORS,'resolution rows are not exact selected locator order')
    source_files=set(); numbers=[]
    for i,row in enumerate(rows,1):
        loc=row['source_row_locator']; source=classifier[loc]; n=source_row_number(loc); numbers.append(n)
        require(row['schema_version']==SCHEMA_VERSION,f'row {i} schema mismatch'); require(row['resolution_id']==f'resolution.external_m07.coordinate_transform_frame_time_policy_wave1.{n:04d}',f'row {i} resolution ID mismatch'); require(row['source_artifact_id']==SOURCE_ARTIFACT_ID,f'row {i} source artifact mismatch'); require(row['classifier_path']==CLASSIFIER_PATH,f'row {i} classifier path mismatch'); require(row['source_row_number']==str(n),f'row {i} source row mismatch')
        for field,cf in [('rust_function_alias','rust_function_alias'),('scilab_function_alias','scilab_function_alias_if_known'),('source_file_locator','source_file_locator'),('formula_family','formula_family'),('risk_tier','risk_tier'),('recommended_chunk_group','recommended_chunk_group')]: require(row[field]==source[cf],f'row {i} classifier mismatch for {field}')
        require(row['validation_status']=='research_required',f'row {i} validation status mismatch'); require(row['disposition']==source['block_reason'],f'row {i} disposition mismatch'); require(row['block_reason']==BLOCK_TEXT[source['block_reason']],f'row {i} block reason mismatch'); source_files.add(row['source_file_locator'])
        for field in ['target_formula_id','target_resolution_id','target_batch_manifest','target_package','target_crate_name','target_runtime_symbol','target_runtime_path','target_contract_path','target_validation_card_path','target_source_seed_path']: require(row[field]=='',f'row {i} blocked disposition must leave {field} empty')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t'); executable=[r for r in inventory if r['category']=='executable_research_equation']; metadata=[r for r in inventory if r['category']=='metadata_only_formula_vault_candidate']; total,backlog,_=external_resolution_inventory(repo,inventory,len(metadata)); require(len(executable)==EXPECTED_EXECUTABLE_ROWS,f'executable count mismatch: {len(executable)}'); require(len(metadata)==EXPECTED_METADATA_ROWS,f'metadata count mismatch: {len(metadata)}'); require(total==EXPECTED_CUMULATIVE_PROCESSED,f'processed count mismatch: {total}'); require(backlog==EXPECTED_REMAINING_BACKLOG,f'backlog count mismatch: {backlog}')
    return {'schema_version':SCHEMA_VERSION,'result':'PASS','wave_id':WAVE_ID,'candidate_pool_rows':len(candidate_pool),'classifier_rows_selected':EXPECTED_ROWS,'candidate_pool_remaining_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'terminal_disposition_rows':len(rows),'deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':40,'risk_tier_counts':dict(sorted(risk.items())),'formula_family_counts':dict(sorted(family.items())),'source_group_counts':dict(sorted(groups.items())),'block_reason_counts':dict(sorted(block.items())),'target_formula_counts':{},'distinct_source_files':len(source_files),'risk_tier_not_downgraded':True,'executable_research_equations':len(executable),'metadata_inventory_records':len(metadata),'external_m07_processed_rows':total,'external_m07_backlog_rows':backlog,'formula_count_delta':0,'runtime_kernel_files_changed':0,'new_validation_cards_required':0,'new_source_seeds_required':0,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True,**validation_contract_fields([])}
def validation_contract_fields(errors:list[str]|None=None)->dict[str,Any]:
    return {'validator':VALIDATOR_NAME,'wave_identifier':WAVE_ID,'selected_row_range':SELECTED_ROW_RANGE,'selected_row_count':EXPECTED_ROWS,'processed_backlog_counters':{'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG},'validation_checks_summary':{'supports_self_test':True,'supports_repo_argument':True,'dependency_free_python':True,'mutates_repository_files':False,'selected_rows':'row_0013 through row_0404 governed-family slice','deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':40,'risk_tier_counts':dict(sorted(EXPECTED_RISK_COUNTS.items())),'formula_family_counts':dict(sorted(EXPECTED_FAMILY_COUNTS.items()))},'errors':errors or []}
def self_test()->dict[str,Any]:
    errors=[]
    if EXPECTED_ROWS!=40: errors.append('EXPECTED_ROWS mismatch')
    if EXPECTED_CUMULATIVE_PROCESSED!=1125: errors.append('EXPECTED_CUMULATIVE_PROCESSED mismatch')
    if EXPECTED_REMAINING_BACKLOG!=198: errors.append('EXPECTED_REMAINING_BACKLOG mismatch')
    if sum(EXPECTED_RISK_COUNTS.values())!=EXPECTED_ROWS: errors.append('risk tier total mismatch')
    if sum(EXPECTED_BLOCK_REASON_COUNTS.values())!=EXPECTED_ROWS: errors.append('block reason total mismatch')
    if len(set(SELECTED_LOCATORS))!=EXPECTED_ROWS: errors.append('selected locators not unique')
    return {'schema_version':SCHEMA_VERSION,'result':'PASS' if not errors else 'FAIL',**validation_contract_fields(errors)}
def emit(payload:dict[str,Any],ok:bool)->int:
    print(stable_json(payload),end=''); return 0 if ok and payload.get('result')=='PASS' else 1
def main()->int:
    p=argparse.ArgumentParser(description=__doc__); p.add_argument('--repo',type=Path,default=Path('.')); p.add_argument('--self-test',action='store_true'); a=p.parse_args()
    try:
        payload=self_test() if a.self_test else verify_repo(a.repo); return emit(payload,payload.get('result')=='PASS')
    except Exception as e:
        payload={'schema_version':SCHEMA_VERSION,'result':'FAIL',**validation_contract_fields([str(e)]),'error':str(e)}; print(stable_json(payload),end='',file=sys.stderr); return 1
if __name__=='__main__': raise SystemExit(main())
