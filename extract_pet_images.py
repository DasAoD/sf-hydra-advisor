#!/usr/bin/env python3
"""
SF Hydra Advisor - Pet Image Extractor
=======================================
Extrahiert alle 100 Pet-Bilder aus einer HAR-Datei des Shakes & Fidget Spiels.

Verwendung:
  1. Im Browser sfgame.net aufrufen und einloggen
  2. F12 → Network-Tab → alle Pets einmal anklicken
  3. HAR-Datei exportieren (Rechtsklick → "Save all as HAR")
  4. Script ausführen:
     python extract_pet_images.py <pfad-zur-har-datei>

Voraussetzungen:
  pip install UnityPy Pillow
"""

import sys
import os
import json
import base64
import urllib.request
import tempfile

def extract_pets(har_path: str, output_dir: str = None):
    if output_dir is None:
        # Standard: dist/assets/pets/ relativ zum Script
        script_dir = os.path.dirname(os.path.abspath(__file__))
        output_dir = os.path.join(script_dir, "dist", "assets", "pets")

    os.makedirs(output_dir, exist_ok=True)

    print(f"Lese HAR-Datei: {har_path}")
    with open(har_path, encoding='utf-8') as f:
        har = json.load(f)

    # CDN-URL für petmediumsprites aus HAR extrahieren
    sprite_url = None
    for entry in har['log']['entries']:
        url = entry['request']['url']
        if 'petmediumsprites' in url:
            sprite_url = url
            break

    if not sprite_url:
        print("FEHLER: petmediumsprites nicht in HAR gefunden.")
        print("Stelle sicher dass du alle Pets angeklickt hast.")
        sys.exit(1)

    print(f"Lade Sprite-Bundle: {sprite_url}")
    with tempfile.NamedTemporaryFile(delete=False, suffix='.bundle') as tmp:
        tmp_path = tmp.name

    try:
        urllib.request.urlretrieve(sprite_url, tmp_path)
        print(f"Bundle geladen ({os.path.getsize(tmp_path) // 1024} KB)")
    except Exception as e:
        print(f"FEHLER beim Laden: {e}")
        sys.exit(1)

    try:
        import UnityPy
        from PIL import Image
    except ImportError:
        print("FEHLER: Bitte installiere die Abhängigkeiten:")
        print("  pip install UnityPy Pillow")
        os.unlink(tmp_path)
        sys.exit(1)

    print("Extrahiere Sprites...")
    env = UnityPy.load(tmp_path)

    # Sprites indizieren
    sprites = {}
    for obj in env.objects:
        if obj.type.name == "Sprite":
            data = obj.read()
            sprites[data.m_Name] = data

    # MonoBehaviour für Reihenfolge lesen
    mono = None
    for obj in env.objects:
        if obj.type.name == "MonoBehaviour":
            data = obj.read()
            if data.m_Name == "PetMediumSprites":
                mono = data
                break

    if not mono:
        print("FEHLER: PetMediumSprites MonoBehaviour nicht gefunden.")
        os.unlink(tmp_path)
        sys.exit(1)

    saved = 0
    for i, pptr in enumerate(mono.petSmall):
        pet_id = i + 1
        try:
            sprite = pptr.read()
            img = sprite.image
            out_path = os.path.join(output_dir, f"pet_{pet_id:03d}.png")
            img.save(out_path)
            saved += 1
        except Exception as e:
            print(f"  Pet {pet_id}: Fehler - {e}")

    os.unlink(tmp_path)
    print(f"\nFertig! {saved}/100 Bilder gespeichert nach: {output_dir}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    extract_pets(sys.argv[1])
