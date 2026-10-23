# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
"""Linux-runnable contract checks; installer identity stays covered without platform-specific install workflows."""
import re
import tomllib
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
NS = {'w': 'http://wixtoolset.org/schemas/v4/wxs'}


class WindowsPackageContract(unittest.TestCase):
    def test_alpha_version_and_install_contract(self):
        version = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['package']['version']
        self.assertEqual(version, '0.12.0-nifty-alpha.1')
        wix = ET.parse(ROOT / 'packaging/windows/designcraft.wxs')
        self.assertEqual(wix.find('w:Package', NS).attrib['Name'], 'NiftyDesign $(var.DisplayVersion)')
        self.assertEqual(wix.find(".//w:Directory[@Id='INSTALLFOLDER']", NS).attrib['Name'], 'NiftyDesign')
        self.assertEqual(wix.find(".//w:File[@Id='DesigncraftExe']", NS).attrib['Source'], '$(var.BinDir)\\designcraft.exe')
        self.assertEqual(wix.find(".//w:Shortcut[@Id='StartMenuShortcut']", NS).attrib['Name'], 'NiftyDesign')
        for node in wix.findall('.//w:RegistryValue', NS):
            if node.attrib.get('Name') == 'FriendlyAppName':
                self.assertEqual(node.attrib['Value'], 'NiftyDesign')
        package = (ROOT / 'packaging/windows/package.ps1').read_text()
        self.assertIn('-d "DisplayVersion=$Version"', package)
        self.assertIn('niftydesign-$Version-windows-$Arch.msi', package)
        build = (ROOT / 'apps/designcraft/build.rs').read_text()
        for key in ('FileVersion', 'ProductVersion'):
            self.assertIn(f'.set("{key}", env!("CARGO_PKG_VERSION"))', build)
        about = (ROOT / 'crates/ui-egui/src/about.rs').read_text()
        self.assertIn('env!("CARGO_PKG_VERSION")', about)
        lock = tomllib.loads((ROOT / 'Cargo.lock').read_text())
        for package in lock['package']:
            if re.fullmatch(r'designcraft(?:-.*)?|xtask', package['name']):
                self.assertEqual(package['version'], version, package['name'])
        self.assertFalse(list((ROOT / 'assets/nifty').glob('*-*-*-*-*.png')))


if __name__ == '__main__':
    unittest.main()
