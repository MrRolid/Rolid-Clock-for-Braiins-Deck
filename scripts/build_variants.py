import copy, json, subprocess, sys
from pathlib import Path
root=Path(__file__).resolve().parents[1]
base=json.loads((root/'manifest.json').read_text())
build=root/'build-manifests';build.mkdir(exist_ok=True)
for lang in ['cz','en','de','es']:
    m=copy.deepcopy(base)
    if lang!='cz':
        c=json.loads((root/f'locales/{lang}.json').read_text())
        m['description']=c['description']
        assert set(c['names']) == set(m['params'])-{'language'}
        for key,param in m['params'].items():
            if key=='language':continue
            param['name']=c['names'][key]
            if 'description' in param:param['description']=c['descriptions'][key]
        m['params']['language']['description']=c['language_description']
        fields=m['params']['alarms']['items']['fields']
        for key,name in c['fields'].items():fields[key]['name']=name
        fields['repeat']['default_value']='Mon-Fri'
        fields['name']['default_value']=c['alarm_default']
        for entry,label in zip(m['params']['led_effect']['enum_values'],c['effects']):entry['label']=label
    m['params']['language']['default_value']=lang
    manifest=build/f'{lang}.json';manifest.write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n')
    subprocess.run([str(root/'target/debug/rolid-manifest-check'),str(manifest)],check=True)
    package=root/f'package-root-{lang}'
    subprocess.run([sys.executable,str(root/'package.py'),str(manifest),str(package)],check=True)
    with (root/f"rolid-clock-{m['version']}-{lang}.nar").open('wb') as f:
        subprocess.run(['nix-store','--dump',str(package)],stdout=f,check=True)
