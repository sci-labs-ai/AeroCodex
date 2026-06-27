#!/usr/bin/env python3
"""Verify A29 external M07 solver / numerical propagation policy Wave 2 terminal dispositions.

Dependency-free metadata verifier. It consumes only governed classifier metadata,
external resolution manifests, and the repository inventory; it never opens raw
Rust-port, M07, or Scilab source text.
"""
from __future__ import annotations
import argparse,csv,json,re,sys
from collections import Counter
from pathlib import Path
from typing import Any,Iterable

SCHEMA_VERSION='aerocodex.external_m07_resolution.v1'
CLASSIFIER_PATH='docs/source_intake/m07_formula_family_classifier/m07_formula_family_classifier.csv'
RESOLUTION_PATH='formula-vault/resolutions/m07_solver_policy_wave2.tsv'
future_same_pool_resolution_paths={
  'formula-vault/resolutions/m07_solver_policy_wave3.tsv'
}
INVENTORY_PATH='validation/equation_inventory.tsv'
SOURCE_ARTIFACT_ID='stage4.m07_rust_port_v14.2026_06_15'
SELECTED_LOCATORS=[
  "PORT_STATUS_RELEASE_GATE.csv:row_0416",
  "PORT_STATUS_RELEASE_GATE.csv:row_0479",
  "PORT_STATUS_RELEASE_GATE.csv:row_0497",
  "PORT_STATUS_RELEASE_GATE.csv:row_0499",
  "PORT_STATUS_RELEASE_GATE.csv:row_0500",
  "PORT_STATUS_RELEASE_GATE.csv:row_0515",
  "PORT_STATUS_RELEASE_GATE.csv:row_0516",
  "PORT_STATUS_RELEASE_GATE.csv:row_0517",
  "PORT_STATUS_RELEASE_GATE.csv:row_0518",
  "PORT_STATUS_RELEASE_GATE.csv:row_0520",
  "PORT_STATUS_RELEASE_GATE.csv:row_0521",
  "PORT_STATUS_RELEASE_GATE.csv:row_0522",
  "PORT_STATUS_RELEASE_GATE.csv:row_0523",
  "PORT_STATUS_RELEASE_GATE.csv:row_0525",
  "PORT_STATUS_RELEASE_GATE.csv:row_0526",
  "PORT_STATUS_RELEASE_GATE.csv:row_0527",
  "PORT_STATUS_RELEASE_GATE.csv:row_0528",
  "PORT_STATUS_RELEASE_GATE.csv:row_0532",
  "PORT_STATUS_RELEASE_GATE.csv:row_0533",
  "PORT_STATUS_RELEASE_GATE.csv:row_0538",
  "PORT_STATUS_RELEASE_GATE.csv:row_0539",
  "PORT_STATUS_RELEASE_GATE.csv:row_0540",
  "PORT_STATUS_RELEASE_GATE.csv:row_0541",
  "PORT_STATUS_RELEASE_GATE.csv:row_0542",
  "PORT_STATUS_RELEASE_GATE.csv:row_0544",
  "PORT_STATUS_RELEASE_GATE.csv:row_0545",
  "PORT_STATUS_RELEASE_GATE.csv:row_0547",
  "PORT_STATUS_RELEASE_GATE.csv:row_0560",
  "PORT_STATUS_RELEASE_GATE.csv:row_0577",
  "PORT_STATUS_RELEASE_GATE.csv:row_0579",
  "PORT_STATUS_RELEASE_GATE.csv:row_0703",
  "PORT_STATUS_RELEASE_GATE.csv:row_0704",
  "PORT_STATUS_RELEASE_GATE.csv:row_0710",
  "PORT_STATUS_RELEASE_GATE.csv:row_0842",
  "PORT_STATUS_RELEASE_GATE.csv:row_0843",
  "PORT_STATUS_RELEASE_GATE.csv:row_0848",
  "PORT_STATUS_RELEASE_GATE.csv:row_0849",
  "PORT_STATUS_RELEASE_GATE.csv:row_0879",
  "PORT_STATUS_RELEASE_GATE.csv:row_0883",
  "PORT_STATUS_RELEASE_GATE.csv:row_0884"
]
CANDIDATE_GROUPS=[
  "9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy",
  "9C_or_10B_generic_numerical_method_policy",
  "9C_solver_rank_tolerance_and_observation_policy",
  "9C_solver_rank_tolerance_policy_before_any_promotion"
]
M07_REPRESENTED_FUNCTION_ROWS=1350
EXPECTED_CANDIDATE_POOL_ROWS=83
EXPECTED_ROWS=40
EXPECTED_REMAINING_CANDIDATE_POOL_ROWS=43
EXPECTED_EXECUTABLE_ROWS=152
EXPECTED_METADATA_ROWS=27
EXPECTED_CUMULATIVE_PROCESSED=855
EXPECTED_REMAINING_BACKLOG=468
EXPECTED_RISK_COUNTS=Counter({'blocked_until_solver_policy': 40})
EXPECTED_FAMILY_COUNTS=Counter({'iterative_solver': 40})
EXPECTED_SOURCE_GROUP_COUNTS=Counter({'9C_kepler_lambert_gauss_solver_policy_or_10B_numerical_propagation_policy': 26, '9C_or_10B_generic_numerical_method_policy': 14})
EXPECTED_BLOCK_REASON_COUNTS=Counter({'blocked_until_iteration_root_selection_and_equivalence_policy': 26, 'blocked_until_numerical_algorithm_tolerance_and_validation_policy': 14})
BLOCK_TEXT={'blocked_until_iteration_root_selection_and_equivalence_policy': 'Classifier row remains blocked until solver iteration limits, root selection, branch handling, convergence failure states, tolerance policy, source registry, and independent equivalence oracles are explicitly approved; no runtime alias or implementation claim is made in A29.', 'blocked_until_numerical_algorithm_tolerance_and_validation_policy': 'Classifier row remains blocked until numerical algorithm tolerance policy, step-size or integration-order policy, stability and failure-state semantics, source registry, and independent numerical oracles are explicitly approved; no runtime alias or implementation claim is made in A29.'}
EXPECTED_HEADER=['schema_version', 'resolution_id', 'source_artifact_id', 'classifier_path', 'source_row_locator', 'source_row_number', 'rust_function_alias', 'scilab_function_alias', 'source_file_locator', 'formula_family', 'risk_tier', 'recommended_chunk_group', 'target_formula_id', 'target_resolution_id', 'target_batch_manifest', 'target_package', 'target_crate_name', 'target_runtime_symbol', 'target_runtime_path', 'target_contract_path', 'target_validation_card_path', 'target_source_seed_path', 'validation_status', 'disposition', 'block_reason']

class VerificationError(RuntimeError): pass
def require(condition:bool,message:str)->None:
    if not condition: raise VerificationError(message)
def stable_json(value:Any)->str:
    return json.dumps(value,indent=2,sort_keys=True,ensure_ascii=False)+'\n'
def read_delimited(path:Path,delimiter:str=',',expected_header:list[str]|None=None)->list[dict[str,str]]:
    require(path.is_file(),f'missing file: {path}')
    with path.open(encoding='utf-8-sig',newline='') as f:
        reader=csv.DictReader(f,delimiter=delimiter)
        require(reader.fieldnames is not None,f'missing header: {path}')
        if expected_header is not None:
            require(reader.fieldnames==expected_header,f'header mismatch: {path}')
        return list(reader)
def repo_file(repo:Path,relative:str)->Path:
    p=(repo/relative).resolve()
    require(repo==p or repo in p.parents,f'path escapes repo: {relative}')
    require(p.is_file(),f'missing repository file: {relative}')
    return p
def source_row_number(locator:str)->int:
    m=re.fullmatch(r'PORT_STATUS_RELEASE_GATE\.csv:row_(\d+)',locator)
    require(m is not None,f'invalid source row locator: {locator}')
    return int(m.group(1))
def unique_map(rows:Iterable[dict[str,str]],key:str,label:str)->dict[str,dict[str,str]]:
    out:dict[str,dict[str,str]]={}
    for row in rows:
        value=row[key]
        require(value not in out,f'duplicate {label} key: {value}')
        out[value]=row
    return out
def prior_external_locators(repo:Path)->set[str]:
    locators:set[str]=set()
    for path in sorted((repo/'formula-vault/resolutions').glob('m07_*.tsv')):
        rel=path.relative_to(repo).as_posix()
        if rel==RESOLUTION_PATH or rel in future_same_pool_resolution_paths:
            continue
        for row in read_delimited(path,'\t',EXPECTED_HEADER):
            locators.add(row['source_row_locator'])
    return locators
def candidate_rows(repo:Path)->list[dict[str,str]]:
    classifier=read_delimited(repo_file(repo,CLASSIFIER_PATH))
    prior=prior_external_locators(repo)
    candidates=[
        row for row in classifier
        if row['recommended_chunk_group'] in CANDIDATE_GROUPS
        and row['m07_row_id_or_alias'] not in prior
    ]
    candidates=sorted(candidates,key=lambda r:source_row_number(r['m07_row_id_or_alias']))
    require(len(candidates)==EXPECTED_CANDIDATE_POOL_ROWS,f'candidate pool count mismatch: {len(candidates)}')
    return candidates
def verify_repo(repo:Path)->dict[str,Any]:
    repo=repo.resolve()
    candidates=candidate_rows(repo)
    selected_expected=[row['m07_row_id_or_alias'] for row in candidates[:EXPECTED_ROWS]]
    require(selected_expected==SELECTED_LOCATORS,'selected row locators do not match first source-ordered candidate slice')
    require(len(candidates[EXPECTED_ROWS:])==EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,'remaining candidate pool mismatch')
    resolution_rows=read_delimited(repo_file(repo,RESOLUTION_PATH),'\t',EXPECTED_HEADER)
    require(len(resolution_rows)==EXPECTED_ROWS,f'resolution row count mismatch: {len(resolution_rows)}')
    resolution_by_locator=unique_map(resolution_rows,'source_row_locator','resolution locator')
    classifier_by_locator=unique_map(read_delimited(repo_file(repo,CLASSIFIER_PATH)),'m07_row_id_or_alias','classifier locator')
    for locator in SELECTED_LOCATORS:
        require(locator in resolution_by_locator,f'missing resolution row: {locator}')
        res=resolution_by_locator[locator]
        cls=classifier_by_locator[locator]
        require(res['schema_version']==SCHEMA_VERSION,f'schema mismatch: {locator}')
        require(res['source_artifact_id']==SOURCE_ARTIFACT_ID,f'source artifact mismatch: {locator}')
        require(res['classifier_path']==CLASSIFIER_PATH,f'classifier path mismatch: {locator}')
        require(int(res['source_row_number'])==source_row_number(locator),f'source row number mismatch: {locator}')
        for key in ['rust_function_alias','source_file_locator','formula_family','risk_tier','recommended_chunk_group']:
            require(res[key]==cls[key],f'{key} mismatch: {locator}')
        require(res['scilab_function_alias']==cls.get('scilab_function_alias_if_known',''),f'scilab alias mismatch: {locator}')
        require(res['validation_status']=='research_required',f'validation status mismatch: {locator}')
        require(res['disposition']==cls['block_reason'],f'disposition/block reason code mismatch: {locator}')
        require(res['block_reason']==BLOCK_TEXT[res['disposition']],f'block text mismatch: {locator}')
        for target_key in ['target_formula_id','target_resolution_id','target_batch_manifest','target_package','target_crate_name','target_runtime_symbol','target_runtime_path','target_contract_path','target_validation_card_path','target_source_seed_path']:
            require(res[target_key]=='',f'target field unexpectedly populated: {locator} {target_key}')
    risk_counts=Counter(row['risk_tier'] for row in resolution_rows)
    family_counts=Counter(row['formula_family'] for row in resolution_rows)
    source_group_counts=Counter(row['recommended_chunk_group'] for row in resolution_rows)
    block_reason_counts=Counter(row['disposition'] for row in resolution_rows)
    require(risk_counts==EXPECTED_RISK_COUNTS,f'risk counts mismatch: {risk_counts}')
    require(family_counts==EXPECTED_FAMILY_COUNTS,f'family counts mismatch: {family_counts}')
    require(source_group_counts==EXPECTED_SOURCE_GROUP_COUNTS,f'source group counts mismatch: {source_group_counts}')
    require(block_reason_counts==EXPECTED_BLOCK_REASON_COUNTS,f'block counts mismatch: {block_reason_counts}')
    inventory=read_delimited(repo_file(repo,INVENTORY_PATH),'\t')
    processed={row['source_path']:row for row in inventory if row['category']=='external_m07_processed_row'}
    require(RESOLUTION_PATH in processed,'new resolution missing from external processed inventory')
    require(int(processed[RESOLUTION_PATH]['row_count'])==EXPECTED_ROWS,'new processed inventory count mismatch')
    backlog_rows=[row for row in inventory if row['category']=='external_m07_backlog_row']
    require(len(backlog_rows)==1,'expected one external backlog aggregate row')
    require(int(backlog_rows[0]['row_count'])==EXPECTED_REMAINING_BACKLOG,'external backlog count mismatch')
    processed_total=sum(int(row['row_count']) for row in processed.values())
    require(processed_total==EXPECTED_CUMULATIVE_PROCESSED,f'cumulative processed mismatch: {processed_total}')
    return {
        'schema_version':'aerocodex.external_m07.solver_policy_wave2.verifier.v1',
        'result':'PASS',
        'resolution_path':RESOLUTION_PATH,
        'selected_rows':SELECTED_LOCATORS,
        'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,
        'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,
        'terminal_disposition_rows':EXPECTED_ROWS,
        'source_group_counts':dict(sorted(source_group_counts.items())),
        'risk_tier_counts':dict(sorted(risk_counts.items())),
        'formula_family_counts':dict(sorted(family_counts.items())),
        'block_reason_counts':dict(sorted(block_reason_counts.items())),
        'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,
        'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,
        'metadata_inventory_records':EXPECTED_METADATA_ROWS,
        'executable_research_equations':EXPECTED_EXECUTABLE_ROWS,
        'validation_status':'research_required',
        'no_rust_m07_or_scilab_source_scraping':True,
        'no_runtime_kernel_change_claim':True,
        'no_external_parity_claim':True,
        'no_certification_or_operational_readiness_claim':True,
    }
def self_test()->dict[str,Any]:
    require(source_row_number('PORT_STATUS_RELEASE_GATE.csv:row_0416')==416,'row parser failed')
    require(stable_json({'b':2,'a':1}).startswith('{\n  "a"'),'stable JSON ordering failed')
    require(len(SELECTED_LOCATORS)==EXPECTED_ROWS,'self-test selected count mismatch')
    require(SELECTED_LOCATORS[0]=='PORT_STATUS_RELEASE_GATE.csv:row_0416','self-test first locator mismatch')
    require(SELECTED_LOCATORS[-1]=='PORT_STATUS_RELEASE_GATE.csv:row_0884','self-test last locator mismatch')
    require(EXPECTED_CUMULATIVE_PROCESSED==826,'processed counter mismatch')
    require(EXPECTED_REMAINING_BACKLOG==497,'backlog counter mismatch')
    return {
        'schema_version':'aerocodex.external_m07.solver_policy_wave2.self_test.v1',
        'result':'PASS',
        'selected_count':len(SELECTED_LOCATORS),
        'candidate_pool_rows':EXPECTED_CANDIDATE_POOL_ROWS,
        'remaining_candidate_pool_rows':EXPECTED_REMAINING_CANDIDATE_POOL_ROWS,
        'external_m07_processed_rows':EXPECTED_CUMULATIVE_PROCESSED,
        'external_m07_backlog_rows':EXPECTED_REMAINING_BACKLOG,
    }
def main()->int:
    parser=argparse.ArgumentParser(description='Verify A29 external M07 solver / numerical propagation policy Wave 2 metadata.')
    parser.add_argument('--repo',type=Path,help='Repository root to validate.')
    parser.add_argument('--self-test',action='store_true',help='Run dependency-free verifier self-test.')
    args=parser.parse_args()
    try:
        if args.self_test:
            report=self_test()
        else:
            repo=args.repo or Path.cwd()
            report=verify_repo(repo)
        sys.stdout.write(stable_json(report))
        return 0
    except Exception as exc:
        sys.stdout.write(stable_json({'result':'FAIL','error':str(exc),'error_type':type(exc).__name__}))
        return 1
if __name__=='__main__':
    raise SystemExit(main())
