#!/usr/bin/env python3
"""Verify A24 orbital-geometry/conic Wave 9 terminal dispositions.

This dependency-free verifier consumes classifier metadata, A16-A23 manifests,
existing external resolution manifests, and explicit A24 terminal resolution records.
It never opens or parses raw Rust-port, M07, or Scilab source text.
"""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any,Iterable
SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
PRIOR_PATHS=['formula-vault/resolutions/m07_orbital_geometry_conic_wave1.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave2.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave3.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave4.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave5.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave6.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave7.tsv', 'formula-vault/resolutions/m07_orbital_geometry_conic_wave8.tsv']
RESOLUTION_PATH='formula-vault/resolutions/m07_orbital_geometry_conic_wave9.tsv'
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
TARGET_CHUNK='9A_classical_elements_and_9E_mission_design_contracts'
M07_REPRESENTED_FUNCTION_ROWS=1350
EXPECTED_CLASSIFIER_GROUP_ROWS=377
EXPECTED_PRIOR_ROWS=320
EXPECTED_ROWS=40
EXPECTED_GROUP_REMAINING_ROWS=17
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=855
EXPECTED_REMAINING_BACKLOG=468
EXPECTED_RISK_COUNTS=Counter({'medium_risk_requires_contract_review':34,'high_risk_requires_numerical_policy':6})
EXPECTED_DISPOSITIONS=Counter({ 'blocked_until_orbit_geometry_conic_branch_and_validation_policy':40 })
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']
BLOCK_REASON='Classifier row remains blocked until orbit-geometry/conic branch conventions, frame/unit contracts, numerical policy, source registry, and independent validation oracle are explicitly approved; no runtime alias or implementation claim is made in A24.'
class VerificationError(RuntimeError): pass
def require(c:bool,m:str)->None:
    if not c: raise VerificationError(m)
def stable_json(v:Any)->str:return json.dumps(v,indent=2,sort_keys=True,ensure_ascii=False)+'\n'
def read_delimited(path:Path,delimiter:str=',',expected_header:list[str]|None=None)->list[dict[str,str]]:
    require(path.is_file(),f'missing file: {path}')
    with path.open(encoding='utf-8-sig',newline='') as f:
        reader=csv.DictReader(f,delimiter=delimiter);require(reader.fieldnames is not None,f'missing header: {path}')
        if expected_header is not None:require(reader.fieldnames==expected_header,f'header mismatch: {path}')
        return list(reader)
def repo_file(repo:Path,relative:str)->Path:
    p=(repo/relative).resolve();require(repo==p or repo in p.parents,f'path escapes repo: {relative}');require(p.is_file(),f'missing repository file: {relative}');return p
def source_row_number(locator:str)->int:
    m=re.fullmatch(r'PORT_STATUS_RELEASE_GATE\.csv:row_(\d+)',locator);require(m is not None,f'invalid source row locator: {locator}');return int(m.group(1))
def unique_map(rows:Iterable[dict[str,str]],key:str,label:str)->dict[str,dict[str,str]]:
    out={}
    for row in rows:
        value=row[key];require(value and value not in out,f'duplicate or empty {label} {key}: {value!r}');out[value]=row
    return out
def require_logical_source_locator(value:str,row_number:int)->None:
    require(value and not value.startswith(('/', '\\')) and not re.match(r'^[A-Za-z]:[\\/]',value),f'row {row_number} source locator is absolute')
def external_resolution_inventory(repo:Path,inventory:list[dict[str,str]],metadata_count:int)->tuple[int,int]:
    processed=[r for r in inventory if r['category']=='external_m07_processed_row'];backlog=[r for r in inventory if r['category']=='external_m07_backlog_row'];processed_map=unique_map(processed,'source_path','external processed inventory');manifests=sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv'));require(manifests,'no external M07 resolution manifests found');expected_paths=set();total=0;locators=set();resolution_ids=set()
    for path in manifests:
        relative=path.relative_to(repo).as_posix();rows=read_delimited(path,'\t',EXPECTED_HEADER);expected_paths.add(relative);inv=processed_map.get(relative);require(inv is not None,f'missing processed inventory row for {relative}');require(inv['row_count']==str(len(rows)),f'processed inventory count mismatch for {relative}');total+=len(rows)
        for row in rows:
            require(row['source_row_locator'] not in locators,f'duplicate external source row locator: {row["source_row_locator"]}');locators.add(row['source_row_locator']);require(row['resolution_id'] not in resolution_ids,f'duplicate external resolution ID: {row["resolution_id"]}');resolution_ids.add(row['resolution_id'])
    require(set(processed_map)==expected_paths,'processed inventory and external manifests are not exact union');require(len(backlog)==1,f'expected one backlog row, found {len(backlog)}');expected=M07_REPRESENTED_FUNCTION_ROWS-metadata_count-total;require(backlog[0]['row_count']==str(expected),'external backlog count mismatch');return total,expected
def verify_repo(repo:Path)->dict[str,Any]:
    repo=repo.resolve();classifier_rows=read_delimited(repo_file(repo,CLASSIFIER_PATH),',');group=sorted([r for r in classifier_rows if r['recommended_chunk_group']==TARGET_CHUNK and r['formula_family']=='orbit_two_body'],key=lambda r:source_row_number(r['m07_row_id_or_alias']));require(len(group)==EXPECTED_CLASSIFIER_GROUP_ROWS,f'classifier group count mismatch: {len(group)}');unique_map(group,'m07_row_id_or_alias','classifier group')
    prior=[]
    for path in PRIOR_PATHS:prior.extend(read_delimited(repo_file(repo,path),'\t',EXPECTED_HEADER))
    require(len(prior)==EXPECTED_PRIOR_ROWS,f'prior row count mismatch: {len(prior)}');prior_locators={r['source_row_locator'] for r in prior};require(len(prior_locators)==EXPECTED_PRIOR_ROWS,'prior locators are not unique');require(prior_locators=={r['m07_row_id_or_alias'] for r in group[:EXPECTED_PRIOR_ROWS]},'A16-A23 are not exact first-320 selection')
    remaining=[r for r in group if r['m07_row_id_or_alias'] not in prior_locators];selected=remaining[:EXPECTED_ROWS];after=remaining[EXPECTED_ROWS:];require(len(selected)==EXPECTED_ROWS,'selected row count mismatch');require(len(after)==EXPECTED_GROUP_REMAINING_ROWS,f'remaining group count mismatch: {len(after)}');classifier=unique_map(selected,'m07_row_id_or_alias','classifier');risk=Counter(r['risk_tier'] for r in selected);require(risk==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {dict(risk)}')
    for loc,row in classifier.items():require(row['implementation_readiness']=='contract_review_first_no_implementation',f'classifier readiness mismatch: {loc}');require(row['block_reason']=='blocked_until_orbit_geometry_conic_branch_and_validation_policy',f'classifier block reason mismatch: {loc}')
    rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER);require(len(rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(rows)}');resolutions=unique_map(rows,'source_row_locator','resolution');unique_map(rows,'resolution_id','resolution');require(set(resolutions)==set(classifier),'classifier selection and resolution locators are not exact union');dispositions=Counter();numbers=[];source_files=set()
    for i,row in enumerate(rows,1):
        loc=row['source_row_locator'];source=classifier[loc];n=source_row_number(loc);numbers.append(n);require(row['schema_version']==SCHEMA_VERSION,f'row {i} schema mismatch');require(row['resolution_id']==f'resolution.external_m07.orbital_geometry_conic_wave9.{n:04d}',f'row {i} resolution ID mismatch');require(row['source_artifact_id']==SOURCE_ARTIFACT_ID,f'row {i} source artifact mismatch');require(row['classifier_path']==CLASSIFIER_PATH,f'row {i} classifier path mismatch');require(row['source_row_number']==str(n),f'row {i} source row mismatch');require_logical_source_locator(row['source_file_locator'],i)
        for field,cf in [('rust_function_alias','rust_function_alias'),('scilab_function_alias','scilab_function_alias_if_known'),('source_file_locator','source_file_locator'),('formula_family','formula_family'),('risk_tier','risk_tier'),('recommended_chunk_group','recommended_chunk_group')]:require(row[field]==source[cf],f'row {i} classifier mismatch for {field}')
        require(row['validation_status']=='research_required',f'row {i} validation status mismatch');require(row['disposition']=='blocked_until_orbit_geometry_conic_branch_and_validation_policy',f'row {i} disposition mismatch');require(row['block_reason']==BLOCK_REASON,f'row {i} block reason mismatch');dispositions[row['disposition']]+=1;source_files.add(row['source_file_locator'])
        for field in ['target_formula_id','target_resolution_id','target_batch_manifest','target_package','target_crate_name','target_runtime_symbol','target_runtime_path','target_contract_path','target_validation_card_path','target_source_seed_path']:require(row[field]=='',f'row {i} blocked disposition must leave {field} empty')
    require(numbers==sorted(numbers),'rows not deterministic source order');require(dispositions==EXPECTED_DISPOSITIONS,f'disposition counts mismatch: {dict(dispositions)}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t');executable=[r for r in inventory if r['category']=='executable_research_equation'];metadata=[r for r in inventory if r['category']=='metadata_only_formula_vault_candidate'];require(len(executable)==EXPECTED_EXECUTABLE_ROWS,f'executable count mismatch: {len(executable)}');require(len(metadata)==EXPECTED_METADATA_ROWS,f'metadata count mismatch: {len(metadata)}');total,backlog=external_resolution_inventory(repo,inventory,len(metadata));require(total==EXPECTED_CUMULATIVE_PROCESSED,f'processed count mismatch: {total}');require(backlog==EXPECTED_REMAINING_BACKLOG,f'backlog count mismatch: {backlog}')
    return {'schema_version':SCHEMA_VERSION,'result':'PASS','wave_id':'a24_external_m07_orbital_geometry_conic_wave9','prior_group_rows':len(prior),'classifier_rows_selected':len(selected),'classifier_group_remaining_rows':len(after),'terminal_disposition_rows':len(rows),'deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':40,'risk_tier_counts':dict(sorted(risk.items())),'target_formula_counts':{},'distinct_source_files':len(source_files),'risk_tier_not_downgraded':True,'executable_research_equations':len(executable),'metadata_inventory_records':len(metadata),'external_m07_processed_rows':total,'external_m07_backlog_rows':backlog,'formula_count_delta':0,'runtime_kernel_files_changed':0,'new_validation_cards_required':0,'new_source_seeds_required':0,'validation_status':'research_required','no_rust_m07_or_scilab_source_scraping':True,'no_external_parity_claim':True,'no_certification_or_operational_readiness_claim':True}
VALIDATOR_NAME='verify_external_m07_orbital_geometry_conic_wave9'
SELECTED_ROW_RANGE={'first':'PORT_STATUS_RELEASE_GATE.csv:row_0729','last':'PORT_STATUS_RELEASE_GATE.csv:row_0788','count':EXPECTED_ROWS}
def validation_contract_fields(errors:list[str]|None=None)->dict[str,Any]:
    return {'validator':VALIDATOR_NAME,'wave_identifier':'a24_external_m07_orbital_geometry_conic_wave9','selected_row_range':SELECTED_ROW_RANGE,'selected_row_count':EXPECTED_ROWS,'processed_backlog_counters':{'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG},'validation_checks_summary':{'supports_self_test':True,'supports_repo_argument':True,'dependency_free_python':True,'mutates_repository_files':False,'selected_rows':'row_0729 through row_0788','deduplicated_alias_rows':0,'excluded_helper_rows':0,'contract_blocked_rows':40,'risk_tier_counts':dict(sorted(EXPECTED_RISK_COUNTS.items()))},'errors':errors or []}
def self_test()->dict[str,Any]:
    errors=[]
    if EXPECTED_ROWS!=40:errors.append('EXPECTED_ROWS mismatch')
    if EXPECTED_CUMULATIVE_PROCESSED!=746:errors.append('EXPECTED_CUMULATIVE_PROCESSED mismatch')
    if EXPECTED_REMAINING_BACKLOG!=577:errors.append('EXPECTED_REMAINING_BACKLOG mismatch')
    if sum(EXPECTED_RISK_COUNTS.values())!=EXPECTED_ROWS:errors.append('risk tier total mismatch')
    if sum(EXPECTED_DISPOSITIONS.values())!=EXPECTED_ROWS:errors.append('disposition total mismatch')
    payload={'schema_version':SCHEMA_VERSION,'result':'PASS' if not errors else 'FAIL',**validation_contract_fields(errors)}
    return payload
def emit(payload:dict[str,Any],ok:bool)->int:
    print(stable_json(payload),end='')
    return 0 if ok and payload.get('result')=='PASS' else 1
def main()->int:
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--repo',type=Path,default=Path('.'))
    p.add_argument('--self-test',action='store_true')
    a=p.parse_args()
    try:
        if a.self_test:
            payload=self_test();return emit(payload,payload.get('result')=='PASS')
        payload=verify_repo(a.repo);payload.update(validation_contract_fields([]));return emit(payload,True)
    except Exception as e:
        payload={'schema_version':SCHEMA_VERSION,'result':'FAIL',**validation_contract_fields([str(e)]),'error':str(e)}
        print(stable_json(payload),end='',file=sys.stderr)
        return 1
if __name__=='__main__':raise SystemExit(main())
