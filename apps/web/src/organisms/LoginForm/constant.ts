/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Attribut `title` de l'élément `<input>`
/// Utilisé pour indiquer à l'utilisateur la valeur attendue pour
/// un pseudonyme.
export const VALIDATION_NICKNAME_INFO: str = `
Pour qu'un pseudonyme soit considéré comme valide, ses caractères
doivent respecter, un format précis, les conditions suivantes :
    - Il ne doit pas commencer par le caractère '-' ou par un caractère
	  numérique '0..9' ;
    - Il peut contenir les caractères: alphanumériques, 'A..Z', 'a..z',
      '0..9'. Les caractères alphabétiques des langues étrangères sont
      considérés comme valides. Par exemple: le russe, le japonais, etc.
   - Il peut contenir les caractères spéciaux suivants: []\`_^{|}
`.trim();

export const MAXLENGTH_NICKNAME: u8 = 24;

export const PLACEHOLDER_NICKNAME: str = `Pseudonyme (max. ${MAXLENGTH_NICKNAME} caractères)`;
export const PLACEHOLDER_SERVER_PASSWORD: str =
	"Mot de passe serveur (optionnel)";
export const PLACEHOLDER_CHANNELS = "Ajouter un ou plusieurs salons";

export const CONFIRM_DELETE_CHANNEL: str =
	"Voulez-vous vraiment supprimer le salon?";
export const BYPASS_CONFIRM_DELETE_CHANNEL: str =
	"Maintenir la touche `Shift` pour outrepasser la boite de confirmation.";
