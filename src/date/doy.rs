use core::fmt::{self, Display, Formatter};

use enumerable::Enumerable;

use crate::date::{
    Day, Month, Year,
    consts::{DAYS_IN_MONTH, DAYS_IN_NORMAL_MONTHS, DAYS_IN_SANS_CULOTTIDES_LEAP_YEAR},
    leap::LeapSystem,
};

/// Days in a year of the Republican Calendar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enumerable)]
pub enum DayOfYear {
    /// Raisin, day 1 of Vendémiaire, day 1 of Autumn, day 1 of the year.
    Raisin = 1,
    /// Safran, day 2 of Vendémiaire, day 2 of Autumn, day 2 of the year.
    Safran = 2,
    /// Châtaigne, day 3 of Vendémiaire, day 3 of Autumn, day 3 of the year.
    Châtaigne = 3,
    /// Colchique, day 4 of Vendémiaire, day 4 of Autumn, day 4 of the year.
    Colchique = 4,
    /// Cheval, day 5 of Vendémiaire, day 5 of Autumn, day 5 of the year.
    Cheval = 5,
    /// Balsamine, day 6 of Vendémiaire, day 6 of Autumn, day 6 of the year.
    Balsamine = 6,
    /// Carotte, day 7 of Vendémiaire, day 7 of Autumn, day 7 of the year.
    Carotte = 7,
    /// Amaranthe, day 8 of Vendémiaire, day 8 of Autumn, day 8 of the year.
    Amaranthe = 8,
    /// Panais, day 9 of Vendémiaire, day 9 of Autumn, day 9 of the year.
    Panais = 9,
    /// Cuve, day 10 of Vendémiaire, day 10 of Autumn, day 10 of the year.
    Cuve = 10,
    /// Pomme de terre, day 11 of Vendémiaire, day 11 of Autumn, day 11 of the year.
    PommeDeTerre = 11,
    /// Immortelle, day 12 of Vendémiaire, day 12 of Autumn, day 12 of the year.
    Immortelle = 12,
    /// Potiron, day 13 of Vendémiaire, day 13 of Autumn, day 13 of the year.
    Potiron = 13,
    /// Réséda, day 14 of Vendémiaire, day 14 of Autumn, day 14 of the year.
    Réséda = 14,
    /// Âne, day 15 of Vendémiaire, day 15 of Autumn, day 15 of the year.
    Âne = 15,
    /// Belle de nuit, day 16 of Vendémiaire, day 16 of Autumn, day 16 of the year.
    BelleDeNuit = 16,
    /// Citrouille, day 17 of Vendémiaire, day 17 of Autumn, day 17 of the year.
    Citrouille = 17,
    /// Sarrasin, day 18 of Vendémiaire, day 18 of Autumn, day 18 of the year.
    Sarrasin = 18,
    /// Tournesol, day 19 of Vendémiaire, day 19 of Autumn, day 19 of the year.
    Tournesol = 19,
    /// Pressoir, day 20 of Vendémiaire, day 20 of Autumn, day 20 of the year.
    Pressoir = 20,
    /// Chanvre, day 21 of Vendémiaire, day 21 of Autumn, day 21 of the year.
    Chanvre = 21,
    /// Pêche, day 22 of Vendémiaire, day 22 of Autumn, day 22 of the year.
    Pêche = 22,
    /// Navet, day 23 of Vendémiaire, day 23 of Autumn, day 23 of the year.
    Navet = 23,
    /// Amaryllis, day 24 of Vendémiaire, day 24 of Autumn, day 24 of the year.
    Amaryllis = 24,
    /// Bœuf, day 25 of Vendémiaire, day 25 of Autumn, day 25 of the year.
    Bœuf = 25,
    /// Aubergine, day 26 of Vendémiaire, day 26 of Autumn, day 26 of the year.
    Aubergine = 26,
    /// Piment, day 27 of Vendémiaire, day 27 of Autumn, day 27 of the year.
    Piment = 27,
    /// Tomate, day 28 of Vendémiaire, day 28 of Autumn, day 28 of the year.
    Tomate = 28,
    /// Orge, day 29 of Vendémiaire, day 29 of Autumn, day 29 of the year.
    Orge = 29,
    /// Tonneau, day 30 of Vendémiaire, day 30 of Autumn, day 30 of the year.
    Tonneau = 30,
    /// Pomme, day 1 of Brumaire, day 31 of Autumn, day 31 of the year.
    Pomme = 31,
    /// Céleri, day 2 of Brumaire, day 32 of Autumn, day 32 of the year.
    Céleri = 32,
    /// Poire, day 3 of Brumaire, day 33 of Autumn, day 33 of the year.
    Poire = 33,
    /// Betterave, day 4 of Brumaire, day 34 of Autumn, day 34 of the year.
    Betterave = 34,
    /// Oie, day 5 of Brumaire, day 35 of Autumn, day 35 of the year.
    Oie = 35,
    /// Héliotrope, day 6 of Brumaire, day 36 of Autumn, day 36 of the year.
    Héliotrope = 36,
    /// Figue, day 7 of Brumaire, day 37 of Autumn, day 37 of the year.
    Figue = 37,
    /// Scorsonère, day 8 of Brumaire, day 38 of Autumn, day 38 of the year.
    Scorsonère = 38,
    /// Alisier, day 9 of Brumaire, day 39 of Autumn, day 39 of the year.
    Alisier = 39,
    /// Charrue, day 10 of Brumaire, day 40 of Autumn, day 40 of the year.
    Charrue = 40,
    /// Salsifis, day 11 of Brumaire, day 41 of Autumn, day 41 of the year.
    Salsifis = 41,
    /// Mâcre, day 12 of Brumaire, day 42 of Autumn, day 42 of the year.
    Mâcre = 42,
    /// Topinambour, day 13 of Brumaire, day 43 of Autumn, day 43 of the year.
    Topinambour = 43,
    /// Endive, day 14 of Brumaire, day 44 of Autumn, day 44 of the year.
    Endive = 44,
    /// Dindon, day 15 of Brumaire, day 45 of Autumn, day 45 of the year.
    Dindon = 45,
    /// Chervis, day 16 of Brumaire, day 46 of Autumn, day 46 of the year.
    Chervis = 46,
    /// Cresson, day 17 of Brumaire, day 47 of Autumn, day 47 of the year.
    Cresson = 47,
    /// Dentelaire, day 18 of Brumaire, day 48 of Autumn, day 48 of the year.
    Dentelaire = 48,
    /// Grenade, day 19 of Brumaire, day 49 of Autumn, day 49 of the year.
    Grenade = 49,
    /// Herse, day 20 of Brumaire, day 50 of Autumn, day 50 of the year.
    Herse = 50,
    /// Bacchante, day 21 of Brumaire, day 51 of Autumn, day 51 of the year.
    Bacchante = 51,
    /// Azerole, day 22 of Brumaire, day 52 of Autumn, day 52 of the year.
    Azerole = 52,
    /// Garance, day 23 of Brumaire, day 53 of Autumn, day 53 of the year.
    Garance = 53,
    /// Orange, day 24 of Brumaire, day 54 of Autumn, day 54 of the year.
    Orange = 54,
    /// Faisan, day 25 of Brumaire, day 55 of Autumn, day 55 of the year.
    Faisan = 55,
    /// Pistache, day 26 of Brumaire, day 56 of Autumn, day 56 of the year.
    Pistache = 56,
    /// Macjonc, day 27 of Brumaire, day 57 of Autumn, day 57 of the year.
    Macjonc = 57,
    /// Coing, day 28 of Brumaire, day 58 of Autumn, day 58 of the year.
    Coing = 58,
    /// Cormier, day 29 of Brumaire, day 59 of Autumn, day 59 of the year.
    Cormier = 59,
    /// Rouleau, day 30 of Brumaire, day 60 of Autumn, day 60 of the year.
    Rouleau = 60,
    /// Raiponce, day 1 of Frimaire, day 61 of Autumn, day 61 of the year.
    Raiponce = 61,
    /// Turneps, day 2 of Frimaire, day 62 of Autumn, day 62 of the year.
    Turneps = 62,
    /// Chicorée, day 3 of Frimaire, day 63 of Autumn, day 63 of the year.
    Chicorée = 63,
    /// Nèfle, day 4 of Frimaire, day 64 of Autumn, day 64 of the year.
    Nèfle = 64,
    /// Cochon, day 5 of Frimaire, day 65 of Autumn, day 65 of the year.
    Cochon = 65,
    /// Mâche, day 6 of Frimaire, day 66 of Autumn, day 66 of the year.
    Mâche = 66,
    /// Chou-fleur, day 7 of Frimaire, day 67 of Autumn, day 67 of the year.
    ChouFleur = 67,
    /// Miel, day 8 of Frimaire, day 68 of Autumn, day 68 of the year.
    Miel = 68,
    /// Genièvre, day 9 of Frimaire, day 69 of Autumn, day 69 of the year.
    Genièvre = 69,
    /// Pioche, day 10 of Frimaire, day 70 of Autumn, day 70 of the year.
    Pioche = 70,
    /// Cire, day 11 of Frimaire, day 71 of Autumn, day 71 of the year.
    Cire = 71,
    /// Raifort, day 12 of Frimaire, day 72 of Autumn, day 72 of the year.
    Raifort = 72,
    /// Cèdre, day 13 of Frimaire, day 73 of Autumn, day 73 of the year.
    Cèdre = 73,
    /// Sapin, day 14 of Frimaire, day 74 of Autumn, day 74 of the year.
    Sapin = 74,
    /// Chevreuil, day 15 of Frimaire, day 75 of Autumn, day 75 of the year.
    Chevreuil = 75,
    /// Ajonc, day 16 of Frimaire, day 76 of Autumn, day 76 of the year.
    Ajonc = 76,
    /// Cyprès, day 17 of Frimaire, day 77 of Autumn, day 77 of the year.
    Cyprès = 77,
    /// Lierre, day 18 of Frimaire, day 78 of Autumn, day 78 of the year.
    Lierre = 78,
    /// Sabine, day 19 of Frimaire, day 79 of Autumn, day 79 of the year.
    Sabine = 79,
    /// Hoyau, day 20 of Frimaire, day 80 of Autumn, day 80 of the year.
    Hoyau = 80,
    /// Érable à sucre, day 21 of Frimaire, day 81 of Autumn, day 81 of the year.
    ÉrableÀSucre = 81,
    /// Bruyère, day 22 of Frimaire, day 82 of Autumn, day 82 of the year.
    Bruyère = 82,
    /// Roseau, day 23 of Frimaire, day 83 of Autumn, day 83 of the year.
    Roseau = 83,
    /// Oseille, day 24 of Frimaire, day 84 of Autumn, day 84 of the year.
    Oseille = 84,
    /// Grillon, day 25 of Frimaire, day 85 of Autumn, day 85 of the year.
    Grillon = 85,
    /// Pignon, day 26 of Frimaire, day 86 of Autumn, day 86 of the year.
    Pignon = 86,
    /// Liège, day 27 of Frimaire, day 87 of Autumn, day 87 of the year.
    Liège = 87,
    /// Truffe, day 28 of Frimaire, day 88 of Autumn, day 88 of the year.
    Truffe = 88,
    /// Olive, day 29 of Frimaire, day 89 of Autumn, day 89 of the year.
    Olive = 89,
    /// Pelle, day 30 of Frimaire, day 90 of Autumn, day 90 of the year.
    Pelle = 90,
    /// Tourbe, day 1 of Nivôse, day 1 of Winter, day 91 of the year.
    Tourbe = 91,
    /// Houille, day 2 of Nivôse, day 2 of Winter, day 92 of the year.
    Houille = 92,
    /// Bitume, day 3 of Nivôse, day 3 of Winter, day 93 of the year.
    Bitume = 93,
    /// Soufre, day 4 of Nivôse, day 4 of Winter, day 94 of the year.
    Soufre = 94,
    /// Chien, day 5 of Nivôse, day 5 of Winter, day 95 of the year.
    Chien = 95,
    /// Lave, day 6 of Nivôse, day 6 of Winter, day 96 of the year.
    Lave = 96,
    /// Terre végétale, day 7 of Nivôse, day 7 of Winter, day 97 of the year.
    TerreVégétale = 97,
    /// Fumier, day 8 of Nivôse, day 8 of Winter, day 98 of the year.
    Fumier = 98,
    /// Salpêtre, day 9 of Nivôse, day 9 of Winter, day 99 of the year.
    Salpêtre = 99,
    /// Fléau, day 10 of Nivôse, day 10 of Winter, day 100 of the year.
    Fléau = 100,
    /// Granit, day 11 of Nivôse, day 11 of Winter, day 101 of the year.
    Granit = 101,
    /// Argile, day 12 of Nivôse, day 12 of Winter, day 102 of the year.
    Argile = 102,
    /// Ardoise, day 13 of Nivôse, day 13 of Winter, day 103 of the year.
    Ardoise = 103,
    /// Grès, day 14 of Nivôse, day 14 of Winter, day 104 of the year.
    Grès = 104,
    /// Lapin, day 15 of Nivôse, day 15 of Winter, day 105 of the year.
    Lapin = 105,
    /// Silex, day 16 of Nivôse, day 16 of Winter, day 106 of the year.
    Silex = 106,
    /// Marne, day 17 of Nivôse, day 17 of Winter, day 107 of the year.
    Marne = 107,
    /// Pierre à chaux, day 18 of Nivôse, day 18 of Winter, day 108 of the year.
    PierreÀChaux = 108,
    /// Marbre, day 19 of Nivôse, day 19 of Winter, day 109 of the year.
    Marbre = 109,
    /// Van, day 20 of Nivôse, day 20 of Winter, day 110 of the year.
    Van = 110,
    /// Pierre à plâtre, day 21 of Nivôse, day 21 of Winter, day 111 of the year.
    PierreÀPlâtre = 111,
    /// Sel, day 22 of Nivôse, day 22 of Winter, day 112 of the year.
    Sel = 112,
    /// Fer, day 23 of Nivôse, day 23 of Winter, day 113 of the year.
    Fer = 113,
    /// Cuivre, day 24 of Nivôse, day 24 of Winter, day 114 of the year.
    Cuivre = 114,
    /// Chat, day 25 of Nivôse, day 25 of Winter, day 115 of the year.
    Chat = 115,
    /// Étain, day 26 of Nivôse, day 26 of Winter, day 116 of the year.
    Étain = 116,
    /// Plomb, day 27 of Nivôse, day 27 of Winter, day 117 of the year.
    Plomb = 117,
    /// Zinc, day 28 of Nivôse, day 28 of Winter, day 118 of the year.
    Zinc = 118,
    /// Mercure, day 29 of Nivôse, day 29 of Winter, day 119 of the year.
    Mercure = 119,
    /// Crible, day 30 of Nivôse, day 30 of Winter, day 120 of the year.
    Crible = 120,
    /// Lauréole, day 1 of Pluviôse, day 31 of Winter, day 121 of the year.
    Lauréole = 121,
    /// Mousse, day 2 of Pluviôse, day 32 of Winter, day 122 of the year.
    Mousse = 122,
    /// Fragon, day 3 of Pluviôse, day 33 of Winter, day 123 of the year.
    Fragon = 123,
    /// Perce-neige, day 4 of Pluviôse, day 34 of Winter, day 124 of the year.
    PerceNeige = 124,
    /// Taureau, day 5 of Pluviôse, day 35 of Winter, day 125 of the year.
    Taureau = 125,
    /// Laurier-thym, day 6 of Pluviôse, day 36 of Winter, day 126 of the year.
    LaurierThym = 126,
    /// Amadouvier, day 7 of Pluviôse, day 37 of Winter, day 127 of the year.
    Amadouvier = 127,
    /// Mézéréon, day 8 of Pluviôse, day 38 of Winter, day 128 of the year.
    Mézéréon = 128,
    /// Peuplier, day 9 of Pluviôse, day 39 of Winter, day 129 of the year.
    Peuplier = 129,
    /// Coignée, day 10 of Pluviôse, day 40 of Winter, day 130 of the year.
    Coignée = 130,
    /// Ellébore, day 11 of Pluviôse, day 41 of Winter, day 131 of the year.
    Ellébore = 131,
    /// Brocoli, day 12 of Pluviôse, day 42 of Winter, day 132 of the year.
    Brocoli = 132,
    /// Laurier, day 13 of Pluviôse, day 43 of Winter, day 133 of the year.
    Laurier = 133,
    /// Avelinier, day 14 of Pluviôse, day 44 of Winter, day 134 of the year.
    Avelinier = 134,
    /// Vache, day 15 of Pluviôse, day 45 of Winter, day 135 of the year.
    Vache = 135,
    /// Buis, day 16 of Pluviôse, day 46 of Winter, day 136 of the year.
    Buis = 136,
    /// Lichen, day 17 of Pluviôse, day 47 of Winter, day 137 of the year.
    Lichen = 137,
    /// If, day 18 of Pluviôse, day 48 of Winter, day 138 of the year.
    If = 138,
    /// Pulmonaire, day 19 of Pluviôse, day 49 of Winter, day 139 of the year.
    Pulmonaire = 139,
    /// Serpette, day 20 of Pluviôse, day 50 of Winter, day 140 of the year.
    Serpette = 140,
    /// Thlaspi, day 21 of Pluviôse, day 51 of Winter, day 141 of the year.
    Thlaspi = 141,
    /// Thimelé, day 22 of Pluviôse, day 52 of Winter, day 142 of the year.
    Thimelé = 142,
    /// Chiendent, day 23 of Pluviôse, day 53 of Winter, day 143 of the year.
    Chiendent = 143,
    /// Trainasse, day 24 of Pluviôse, day 54 of Winter, day 144 of the year.
    Trainasse = 144,
    /// Lièvre, day 25 of Pluviôse, day 55 of Winter, day 145 of the year.
    Lièvre = 145,
    /// Guède, day 26 of Pluviôse, day 56 of Winter, day 146 of the year.
    Guède = 146,
    /// Noisetier, day 27 of Pluviôse, day 57 of Winter, day 147 of the year.
    Noisetier = 147,
    /// Cyclamen, day 28 of Pluviôse, day 58 of Winter, day 148 of the year.
    Cyclamen = 148,
    /// Chélidoine, day 29 of Pluviôse, day 59 of Winter, day 149 of the year.
    Chélidoine = 149,
    /// Traîneau, day 30 of Pluviôse, day 60 of Winter, day 150 of the year.
    Traîneau = 150,
    /// Tussilage, day 1 of Ventôse, day 61 of Winter, day 151 of the year.
    Tussilage = 151,
    /// Cornouiller, day 2 of Ventôse, day 62 of Winter, day 152 of the year.
    Cornouiller = 152,
    /// Violier, day 3 of Ventôse, day 63 of Winter, day 153 of the year.
    Violier = 153,
    /// Troène, day 4 of Ventôse, day 64 of Winter, day 154 of the year.
    Troène = 154,
    /// Bouc, day 5 of Ventôse, day 65 of Winter, day 155 of the year.
    Bouc = 155,
    /// Asaret, day 6 of Ventôse, day 66 of Winter, day 156 of the year.
    Asaret = 156,
    /// Alaterne, day 7 of Ventôse, day 67 of Winter, day 157 of the year.
    Alaterne = 157,
    /// Violette, day 8 of Ventôse, day 68 of Winter, day 158 of the year.
    Violette = 158,
    /// Marceau, day 9 of Ventôse, day 69 of Winter, day 159 of the year.
    Marceau = 159,
    /// Bêche, day 10 of Ventôse, day 70 of Winter, day 160 of the year.
    Bêche = 160,
    /// Narcisse, day 11 of Ventôse, day 71 of Winter, day 161 of the year.
    Narcisse = 161,
    /// Orme, day 12 of Ventôse, day 72 of Winter, day 162 of the year.
    Orme = 162,
    /// Fumeterre, day 13 of Ventôse, day 73 of Winter, day 163 of the year.
    Fumeterre = 163,
    /// Vélar, day 14 of Ventôse, day 74 of Winter, day 164 of the year.
    Vélar = 164,
    /// Chèvre, day 15 of Ventôse, day 75 of Winter, day 165 of the year.
    Chèvre = 165,
    /// Épinard, day 16 of Ventôse, day 76 of Winter, day 166 of the year.
    Épinard = 166,
    /// Doronic, day 17 of Ventôse, day 77 of Winter, day 167 of the year.
    Doronic = 167,
    /// Mouron, day 18 of Ventôse, day 78 of Winter, day 168 of the year.
    Mouron = 168,
    /// Cerfeuil, day 19 of Ventôse, day 79 of Winter, day 169 of the year.
    Cerfeuil = 169,
    /// Cordeau, day 20 of Ventôse, day 80 of Winter, day 170 of the year.
    Cordeau = 170,
    /// Mandragore, day 21 of Ventôse, day 81 of Winter, day 171 of the year.
    Mandragore = 171,
    /// Persil, day 22 of Ventôse, day 82 of Winter, day 172 of the year.
    Persil = 172,
    /// Cochléaria, day 23 of Ventôse, day 83 of Winter, day 173 of the year.
    Cochléaria = 173,
    /// Pâquerette, day 24 of Ventôse, day 84 of Winter, day 174 of the year.
    Pâquerette = 174,
    /// Thon, day 25 of Ventôse, day 85 of Winter, day 175 of the year.
    Thon = 175,
    /// Pissenlit, day 26 of Ventôse, day 86 of Winter, day 176 of the year.
    Pissenlit = 176,
    /// Sylvie, day 27 of Ventôse, day 87 of Winter, day 177 of the year.
    Sylvie = 177,
    /// Capillaire, day 28 of Ventôse, day 88 of Winter, day 178 of the year.
    Capillaire = 178,
    /// Frêne, day 29 of Ventôse, day 89 of Winter, day 179 of the year.
    Frêne = 179,
    /// Plantoir, day 30 of Ventôse, day 90 of Winter, day 180 of the year.
    Plantoir = 180,
    /// Primevère, day 1 of Germinal, day 1 of Spring, day 181 of the year.
    Primevère = 181,
    /// Platane, day 2 of Germinal, day 2 of Spring, day 182 of the year.
    Platane = 182,
    /// Asperge, day 3 of Germinal, day 3 of Spring, day 183 of the year.
    Asperge = 183,
    /// Tulipe, day 4 of Germinal, day 4 of Spring, day 184 of the year.
    Tulipe = 184,
    /// Poule, day 5 of Germinal, day 5 of Spring, day 185 of the year.
    Poule = 185,
    /// Bette, day 6 of Germinal, day 6 of Spring, day 186 of the year.
    Bette = 186,
    /// Bouleau, day 7 of Germinal, day 7 of Spring, day 187 of the year.
    Bouleau = 187,
    /// Jonquille, day 8 of Germinal, day 8 of Spring, day 188 of the year.
    Jonquille = 188,
    /// Aulne, day 9 of Germinal, day 9 of Spring, day 189 of the year.
    Aulne = 189,
    /// Couvoir, day 10 of Germinal, day 10 of Spring, day 190 of the year.
    Couvoir = 190,
    /// Pervenche, day 11 of Germinal, day 11 of Spring, day 191 of the year.
    Pervenche = 191,
    /// Charme, day 12 of Germinal, day 12 of Spring, day 192 of the year.
    Charme = 192,
    /// Morille, day 13 of Germinal, day 13 of Spring, day 193 of the year.
    Morille = 193,
    /// Hêtre, day 14 of Germinal, day 14 of Spring, day 194 of the year.
    Hêtre = 194,
    /// Abeille, day 15 of Germinal, day 15 of Spring, day 195 of the year.
    Abeille = 195,
    /// Laitue, day 16 of Germinal, day 16 of Spring, day 196 of the year.
    Laitue = 196,
    /// Mélèze, day 17 of Germinal, day 17 of Spring, day 197 of the year.
    Mélèze = 197,
    /// Ciguë, day 18 of Germinal, day 18 of Spring, day 198 of the year.
    Ciguë = 198,
    /// Radis, day 19 of Germinal, day 19 of Spring, day 199 of the year.
    Radis = 199,
    /// Ruche, day 20 of Germinal, day 20 of Spring, day 200 of the year.
    Ruche = 200,
    /// Gainier, day 21 of Germinal, day 21 of Spring, day 201 of the year.
    Gainier = 201,
    /// Romaine, day 22 of Germinal, day 22 of Spring, day 202 of the year.
    Romaine = 202,
    /// Marronnier, day 23 of Germinal, day 23 of Spring, day 203 of the year.
    Marronnier = 203,
    /// Roquette, day 24 of Germinal, day 24 of Spring, day 204 of the year.
    Roquette = 204,
    /// Pigeon, day 25 of Germinal, day 25 of Spring, day 205 of the year.
    Pigeon = 205,
    /// Lilas, day 26 of Germinal, day 26 of Spring, day 206 of the year.
    Lilas = 206,
    /// Anémone, day 27 of Germinal, day 27 of Spring, day 207 of the year.
    Anémone = 207,
    /// Pensée, day 28 of Germinal, day 28 of Spring, day 208 of the year.
    Pensée = 208,
    /// Myrtille, day 29 of Germinal, day 29 of Spring, day 209 of the year.
    Myrtille = 209,
    /// Greffoir, day 30 of Germinal, day 30 of Spring, day 210 of the year.
    Greffoir = 210,
    /// Rose, day 1 of Floréal, day 31 of Spring, day 211 of the year.
    Rose = 211,
    /// Chêne, day 2 of Floréal, day 32 of Spring, day 212 of the year.
    Chêne = 212,
    /// Fougère, day 3 of Floréal, day 33 of Spring, day 213 of the year.
    Fougère = 213,
    /// Aubépine, day 4 of Floréal, day 34 of Spring, day 214 of the year.
    Aubépine = 214,
    /// Rossignol, day 5 of Floréal, day 35 of Spring, day 215 of the year.
    Rossignol = 215,
    /// Ancolie, day 6 of Floréal, day 36 of Spring, day 216 of the year.
    Ancolie = 216,
    /// Muguet, day 7 of Floréal, day 37 of Spring, day 217 of the year.
    Muguet = 217,
    /// Champignon, day 8 of Floréal, day 38 of Spring, day 218 of the year.
    Champignon = 218,
    /// Hyacinthe, day 9 of Floréal, day 39 of Spring, day 219 of the year.
    Hyacinthe = 219,
    /// Râteau, day 10 of Floréal, day 40 of Spring, day 220 of the year.
    Râteau = 220,
    /// Rhubarbe, day 11 of Floréal, day 41 of Spring, day 221 of the year.
    Rhubarbe = 221,
    /// Sainfoin, day 12 of Floréal, day 42 of Spring, day 222 of the year.
    Sainfoin = 222,
    /// Bâton d'or, day 13 of Floréal, day 43 of Spring, day 223 of the year.
    BâtonDOr = 223,
    /// Chamerisier, day 14 of Floréal, day 44 of Spring, day 224 of the year.
    Chamerisier = 224,
    /// Ver à soie, day 15 of Floréal, day 45 of Spring, day 225 of the year.
    VerÀSoie = 225,
    /// Consoude, day 16 of Floréal, day 46 of Spring, day 226 of the year.
    Consoude = 226,
    /// Pimprenelle, day 17 of Floréal, day 47 of Spring, day 227 of the year.
    Pimprenelle = 227,
    /// Corbeille d'or, day 18 of Floréal, day 48 of Spring, day 228 of the year.
    CorbeilleDOr = 228,
    /// Arroche, day 19 of Floréal, day 49 of Spring, day 229 of the year.
    Arroche = 229,
    /// Sarcloir, day 20 of Floréal, day 50 of Spring, day 230 of the year.
    Sarcloir = 230,
    /// Statice, day 21 of Floréal, day 51 of Spring, day 231 of the year.
    Statice = 231,
    /// Fritillaire, day 22 of Floréal, day 52 of Spring, day 232 of the year.
    Fritillaire = 232,
    /// Bourrache, day 23 of Floréal, day 53 of Spring, day 233 of the year.
    Bourrache = 233,
    /// Valériane, day 24 of Floréal, day 54 of Spring, day 234 of the year.
    Valériane = 234,
    /// Carpe, day 25 of Floréal, day 55 of Spring, day 235 of the year.
    Carpe = 235,
    /// Fusain, day 26 of Floréal, day 56 of Spring, day 236 of the year.
    Fusain = 236,
    /// Civette, day 27 of Floréal, day 57 of Spring, day 237 of the year.
    Civette = 237,
    /// Buglosse, day 28 of Floréal, day 58 of Spring, day 238 of the year.
    Buglosse = 238,
    /// Sénevé, day 29 of Floréal, day 59 of Spring, day 239 of the year.
    Sénevé = 239,
    /// Houlette, day 30 of Floréal, day 60 of Spring, day 240 of the year.
    Houlette = 240,
    /// Luzerne, day 1 of Prairial, day 61 of Spring, day 241 of the year.
    Luzerne = 241,
    /// Hémérocalle, day 2 of Prairial, day 62 of Spring, day 242 of the year.
    Hémérocalle = 242,
    /// Trèfle, day 3 of Prairial, day 63 of Spring, day 243 of the year.
    Trèfle = 243,
    /// Angélique, day 4 of Prairial, day 64 of Spring, day 244 of the year.
    Angélique = 244,
    /// Canard, day 5 of Prairial, day 65 of Spring, day 245 of the year.
    Canard = 245,
    /// Mélisse, day 6 of Prairial, day 66 of Spring, day 246 of the year.
    Mélisse = 246,
    /// Fromental, day 7 of Prairial, day 67 of Spring, day 247 of the year.
    Fromental = 247,
    /// Martagon, day 8 of Prairial, day 68 of Spring, day 248 of the year.
    Martagon = 248,
    /// Serpolet, day 9 of Prairial, day 69 of Spring, day 249 of the year.
    Serpolet = 249,
    /// Faux, day 10 of Prairial, day 70 of Spring, day 250 of the year.
    Faux = 250,
    /// Fraise, day 11 of Prairial, day 71 of Spring, day 251 of the year.
    Fraise = 251,
    /// Bétoine, day 12 of Prairial, day 72 of Spring, day 252 of the year.
    Bétoine = 252,
    /// Pois, day 13 of Prairial, day 73 of Spring, day 253 of the year.
    Pois = 253,
    /// Acacia, day 14 of Prairial, day 74 of Spring, day 254 of the year.
    Acacia = 254,
    /// Caille, day 15 of Prairial, day 75 of Spring, day 255 of the year.
    Caille = 255,
    /// Œillet, day 16 of Prairial, day 76 of Spring, day 256 of the year.
    Œillet = 256,
    /// Sureau, day 17 of Prairial, day 77 of Spring, day 257 of the year.
    Sureau = 257,
    /// Pavot, day 18 of Prairial, day 78 of Spring, day 258 of the year.
    Pavot = 258,
    /// Tilleul, day 19 of Prairial, day 79 of Spring, day 259 of the year.
    Tilleul = 259,
    /// Fourche, day 20 of Prairial, day 80 of Spring, day 260 of the year.
    Fourche = 260,
    /// Barbeau, day 21 of Prairial, day 81 of Spring, day 261 of the year.
    Barbeau = 261,
    /// Camomille, day 22 of Prairial, day 82 of Spring, day 262 of the year.
    Camomille = 262,
    /// Chèvrefeuille, day 23 of Prairial, day 83 of Spring, day 263 of the year.
    Chèvrefeuille = 263,
    /// Caille-lait, day 24 of Prairial, day 84 of Spring, day 264 of the year.
    CailleLait = 264,
    /// Tanche, day 25 of Prairial, day 85 of Spring, day 265 of the year.
    Tanche = 265,
    /// Jasmin, day 26 of Prairial, day 86 of Spring, day 266 of the year.
    Jasmin = 266,
    /// Verveine, day 27 of Prairial, day 87 of Spring, day 267 of the year.
    Verveine = 267,
    /// Thym, day 28 of Prairial, day 88 of Spring, day 268 of the year.
    Thym = 268,
    /// Pivoine, day 29 of Prairial, day 89 of Spring, day 269 of the year.
    Pivoine = 269,
    /// Chariot, day 30 of Prairial, day 90 of Spring, day 270 of the year.
    Chariot = 270,
    /// Seigle, day 1 of Messidor, day 1 of Summer, day 271 of the year.
    Seigle = 271,
    /// Avoine, day 2 of Messidor, day 2 of Summer, day 272 of the year.
    Avoine = 272,
    /// Oignon, day 3 of Messidor, day 3 of Summer, day 273 of the year.
    Oignon = 273,
    /// Véronique, day 4 of Messidor, day 4 of Summer, day 274 of the year.
    Véronique = 274,
    /// Mulet, day 5 of Messidor, day 5 of Summer, day 275 of the year.
    Mulet = 275,
    /// Romarin, day 6 of Messidor, day 6 of Summer, day 276 of the year.
    Romarin = 276,
    /// Concombre, day 7 of Messidor, day 7 of Summer, day 277 of the year.
    Concombre = 277,
    /// Échalote, day 8 of Messidor, day 8 of Summer, day 278 of the year.
    Échalote = 278,
    /// Absinthe, day 9 of Messidor, day 9 of Summer, day 279 of the year.
    Absinthe = 279,
    /// Faucille, day 10 of Messidor, day 10 of Summer, day 280 of the year.
    Faucille = 280,
    /// Coriandre, day 11 of Messidor, day 11 of Summer, day 281 of the year.
    Coriandre = 281,
    /// Artichaut, day 12 of Messidor, day 12 of Summer, day 282 of the year.
    Artichaut = 282,
    /// Girofle, day 13 of Messidor, day 13 of Summer, day 283 of the year.
    Girofle = 283,
    /// Lavande, day 14 of Messidor, day 14 of Summer, day 284 of the year.
    Lavande = 284,
    /// Chamois, day 15 of Messidor, day 15 of Summer, day 285 of the year.
    Chamois = 285,
    /// Tabac, day 16 of Messidor, day 16 of Summer, day 286 of the year.
    Tabac = 286,
    /// Groseille, day 17 of Messidor, day 17 of Summer, day 287 of the year.
    Groseille = 287,
    /// Gesse, day 18 of Messidor, day 18 of Summer, day 288 of the year.
    Gesse = 288,
    /// Cerise, day 19 of Messidor, day 19 of Summer, day 289 of the year.
    Cerise = 289,
    /// Parc, day 20 of Messidor, day 20 of Summer, day 290 of the year.
    Parc = 290,
    /// Menthe, day 21 of Messidor, day 21 of Summer, day 291 of the year.
    Menthe = 291,
    /// Cumin, day 22 of Messidor, day 22 of Summer, day 292 of the year.
    Cumin = 292,
    /// Haricot, day 23 of Messidor, day 23 of Summer, day 293 of the year.
    Haricot = 293,
    /// Orcanète, day 24 of Messidor, day 24 of Summer, day 294 of the year.
    Orcanète = 294,
    /// Pintade, day 25 of Messidor, day 25 of Summer, day 295 of the year.
    Pintade = 295,
    /// Sauge, day 26 of Messidor, day 26 of Summer, day 296 of the year.
    Sauge = 296,
    /// Ail, day 27 of Messidor, day 27 of Summer, day 297 of the year.
    Ail = 297,
    /// Vesce, day 28 of Messidor, day 28 of Summer, day 298 of the year.
    Vesce = 298,
    /// Blé, day 29 of Messidor, day 29 of Summer, day 299 of the year.
    Blé = 299,
    /// Chalémie, day 30 of Messidor, day 30 of Summer, day 300 of the year.
    Chalémie = 300,
    /// Épeautre, day 1 of Thermidor, day 31 of Summer, day 301 of the year.
    Épeautre = 301,
    /// Bouillon blanc, day 2 of Thermidor, day 32 of Summer, day 302 of the year.
    BouillonBlanc = 302,
    /// Melon, day 3 of Thermidor, day 33 of Summer, day 303 of the year.
    Melon = 303,
    /// Ivraie, day 4 of Thermidor, day 34 of Summer, day 304 of the year.
    Ivraie = 304,
    /// Bélier, day 5 of Thermidor, day 35 of Summer, day 305 of the year.
    Bélier = 305,
    /// Prêle, day 6 of Thermidor, day 36 of Summer, day 306 of the year.
    Prêle = 306,
    /// Armoise, day 7 of Thermidor, day 37 of Summer, day 307 of the year.
    Armoise = 307,
    /// Carthame, day 8 of Thermidor, day 38 of Summer, day 308 of the year.
    Carthame = 308,
    /// Mûre, day 9 of Thermidor, day 39 of Summer, day 309 of the year.
    Mûre = 309,
    /// Arrosoir, day 10 of Thermidor, day 40 of Summer, day 310 of the year.
    Arrosoir = 310,
    /// Panic, day 11 of Thermidor, day 41 of Summer, day 311 of the year.
    Panic = 311,
    /// Salicorne, day 12 of Thermidor, day 42 of Summer, day 312 of the year.
    Salicorne = 312,
    /// Abricot, day 13 of Thermidor, day 43 of Summer, day 313 of the year.
    Abricot = 313,
    /// Basilic, day 14 of Thermidor, day 44 of Summer, day 314 of the year.
    Basilic = 314,
    /// Brebis, day 15 of Thermidor, day 45 of Summer, day 315 of the year.
    Brebis = 315,
    /// Guimauve, day 16 of Thermidor, day 46 of Summer, day 316 of the year.
    Guimauve = 316,
    /// Lin, day 17 of Thermidor, day 47 of Summer, day 317 of the year.
    Lin = 317,
    /// Amande, day 18 of Thermidor, day 48 of Summer, day 318 of the year.
    Amande = 318,
    /// Gentiane, day 19 of Thermidor, day 49 of Summer, day 319 of the year.
    Gentiane = 319,
    /// Écluse, day 20 of Thermidor, day 50 of Summer, day 320 of the year.
    Écluse = 320,
    /// Carline, day 21 of Thermidor, day 51 of Summer, day 321 of the year.
    Carline = 321,
    /// Câprier, day 22 of Thermidor, day 52 of Summer, day 322 of the year.
    Câprier = 322,
    /// Lentille, day 23 of Thermidor, day 53 of Summer, day 323 of the year.
    Lentille = 323,
    /// Aunée, day 24 of Thermidor, day 54 of Summer, day 324 of the year.
    Aunée = 324,
    /// Loutre, day 25 of Thermidor, day 55 of Summer, day 325 of the year.
    Loutre = 325,
    /// Myrte, day 26 of Thermidor, day 56 of Summer, day 326 of the year.
    Myrte = 326,
    /// Colza, day 27 of Thermidor, day 57 of Summer, day 327 of the year.
    Colza = 327,
    /// Lupin, day 28 of Thermidor, day 58 of Summer, day 328 of the year.
    Lupin = 328,
    /// Coton, day 29 of Thermidor, day 59 of Summer, day 329 of the year.
    Coton = 329,
    /// Moulin, day 30 of Thermidor, day 60 of Summer, day 330 of the year.
    Moulin = 330,
    /// Prune, day 1 of Fructidor, day 61 of Summer, day 331 of the year.
    Prune = 331,
    /// Millet, day 2 of Fructidor, day 62 of Summer, day 332 of the year.
    Millet = 332,
    /// Lycoperdon, day 3 of Fructidor, day 63 of Summer, day 333 of the year.
    Lycoperdon = 333,
    /// Escourgeon, day 4 of Fructidor, day 64 of Summer, day 334 of the year.
    Escourgeon = 334,
    /// Saumon, day 5 of Fructidor, day 65 of Summer, day 335 of the year.
    Saumon = 335,
    /// Tubéreuse, day 6 of Fructidor, day 66 of Summer, day 336 of the year.
    Tubéreuse = 336,
    /// Sucrion, day 7 of Fructidor, day 67 of Summer, day 337 of the year.
    Sucrion = 337,
    /// Apocyn, day 8 of Fructidor, day 68 of Summer, day 338 of the year.
    Apocyn = 338,
    /// Réglisse, day 9 of Fructidor, day 69 of Summer, day 339 of the year.
    Réglisse = 339,
    /// Échelle, day 10 of Fructidor, day 70 of Summer, day 340 of the year.
    Échelle = 340,
    /// Pastèque, day 11 of Fructidor, day 71 of Summer, day 341 of the year.
    Pastèque = 341,
    /// Fenouil, day 12 of Fructidor, day 72 of Summer, day 342 of the year.
    Fenouil = 342,
    /// Épine vinette, day 13 of Fructidor, day 73 of Summer, day 343 of the year.
    ÉpineVinette = 343,
    /// Noix, day 14 of Fructidor, day 74 of Summer, day 344 of the year.
    Noix = 344,
    /// Truite, day 15 of Fructidor, day 75 of Summer, day 345 of the year.
    Truite = 345,
    /// Citron, day 16 of Fructidor, day 76 of Summer, day 346 of the year.
    Citron = 346,
    /// Cardère, day 17 of Fructidor, day 77 of Summer, day 347 of the year.
    Cardère = 347,
    /// Nerprun, day 18 of Fructidor, day 78 of Summer, day 348 of the year.
    Nerprun = 348,
    /// Tagette, day 19 of Fructidor, day 79 of Summer, day 349 of the year.
    Tagette = 349,
    /// Hotte, day 20 of Fructidor, day 80 of Summer, day 350 of the year.
    Hotte = 350,
    /// Églantier, day 21 of Fructidor, day 81 of Summer, day 351 of the year.
    Églantier = 351,
    /// Noisette, day 22 of Fructidor, day 82 of Summer, day 352 of the year.
    Noisette = 352,
    /// Houblon, day 23 of Fructidor, day 83 of Summer, day 353 of the year.
    Houblon = 353,
    /// Sorgho, day 24 of Fructidor, day 84 of Summer, day 354 of the year.
    Sorgho = 354,
    /// Écrevisse, day 25 of Fructidor, day 85 of Summer, day 355 of the year.
    Écrevisse = 355,
    /// Bigarade, day 26 of Fructidor, day 86 of Summer, day 356 of the year.
    Bigarade = 356,
    /// Verge d'or, day 27 of Fructidor, day 87 of Summer, day 357 of the year.
    VergeDOr = 357,
    /// Maïs, day 28 of Fructidor, day 88 of Summer, day 358 of the year.
    Maïs = 358,
    /// Marron, day 29 of Fructidor, day 89 of Summer, day 359 of the year.
    Marron = 359,
    /// Panier, day 30 of Fructidor, day 90 of Summer, day 360 of the year.
    Panier = 360,
    /// La fête de la vertu, day 1 of Sans-culottides, day 361 of the year.
    LaFêteDeLaVertu = 361,
    /// La fête du génie, day 2 of Sans-culottides, day 362 of the year.
    LaFêteDuGénie = 362,
    /// La fête du travail, day 3 of Sans-culottides, day 363 of the year.
    LaFêteDuTravail = 363,
    /// La fête de l'opinion, day 4 of Sans-culottides, day 364 of the year.
    LaFêteDeLOpinion = 364,
    /// La fête des récompenses, day 5 of Sans-culottides, day 365 of the year.
    LaFêteDesRécompenses = 365,
    /// La fête de la Révolution, day 6 of Sans-culottides, day 366 of the year.
    LaFêteDeLaRévolution = 366,
}

impl Display for DayOfYear {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let day_str = match self {
            DayOfYear::Raisin => "Raisin",
            DayOfYear::Safran => "Safran",
            DayOfYear::Châtaigne => "Châtaigne",
            DayOfYear::Colchique => "Colchique",
            DayOfYear::Cheval => "Cheval",
            DayOfYear::Balsamine => "Balsamine",
            DayOfYear::Carotte => "Carotte",
            DayOfYear::Amaranthe => "Amaranthe",
            DayOfYear::Panais => "Panais",
            DayOfYear::Cuve => "Cuve",
            DayOfYear::PommeDeTerre => "Pomme de terre",
            DayOfYear::Immortelle => "Immortelle",
            DayOfYear::Potiron => "Potiron",
            DayOfYear::Réséda => "Réséda",
            DayOfYear::Âne => "Âne",
            DayOfYear::BelleDeNuit => "Belle de nuit",
            DayOfYear::Citrouille => "Citrouille",
            DayOfYear::Sarrasin => "Sarrasin",
            DayOfYear::Tournesol => "Tournesol",
            DayOfYear::Pressoir => "Pressoir",
            DayOfYear::Chanvre => "Chanvre",
            DayOfYear::Pêche => "Pêche",
            DayOfYear::Navet => "Navet",
            DayOfYear::Amaryllis => "Amaryllis",
            DayOfYear::Bœuf => "Bœuf",
            DayOfYear::Aubergine => "Aubergine",
            DayOfYear::Piment => "Piment",
            DayOfYear::Tomate => "Tomate",
            DayOfYear::Orge => "Orge",
            DayOfYear::Tonneau => "Tonneau",
            DayOfYear::Pomme => "Pomme",
            DayOfYear::Céleri => "Céleri",
            DayOfYear::Poire => "Poire",
            DayOfYear::Betterave => "Betterave",
            DayOfYear::Oie => "Oie",
            DayOfYear::Héliotrope => "Héliotrope",
            DayOfYear::Figue => "Figue",
            DayOfYear::Scorsonère => "Scorsonère",
            DayOfYear::Alisier => "Alisier",
            DayOfYear::Charrue => "Charrue",
            DayOfYear::Salsifis => "Salsifis",
            DayOfYear::Mâcre => "Mâcre",
            DayOfYear::Topinambour => "Topinambour",
            DayOfYear::Endive => "Endive",
            DayOfYear::Dindon => "Dindon",
            DayOfYear::Chervis => "Chervis",
            DayOfYear::Cresson => "Cresson",
            DayOfYear::Dentelaire => "Dentelaire",
            DayOfYear::Grenade => "Grenade",
            DayOfYear::Herse => "Herse",
            DayOfYear::Bacchante => "Bacchante",
            DayOfYear::Azerole => "Azerole",
            DayOfYear::Garance => "Garance",
            DayOfYear::Orange => "Orange",
            DayOfYear::Faisan => "Faisan",
            DayOfYear::Pistache => "Pistache",
            DayOfYear::Macjonc => "Macjonc",
            DayOfYear::Coing => "Coing",
            DayOfYear::Cormier => "Cormier",
            DayOfYear::Rouleau => "Rouleau",
            DayOfYear::Raiponce => "Raiponce",
            DayOfYear::Turneps => "Turneps",
            DayOfYear::Chicorée => "Chicorée",
            DayOfYear::Nèfle => "Nèfle",
            DayOfYear::Cochon => "Cochon",
            DayOfYear::Mâche => "Mâche",
            DayOfYear::ChouFleur => "Chou-fleur",
            DayOfYear::Miel => "Miel",
            DayOfYear::Genièvre => "Genièvre",
            DayOfYear::Pioche => "Pioche",
            DayOfYear::Cire => "Cire",
            DayOfYear::Raifort => "Raifort",
            DayOfYear::Cèdre => "Cèdre",
            DayOfYear::Sapin => "Sapin",
            DayOfYear::Chevreuil => "Chevreuil",
            DayOfYear::Ajonc => "Ajonc",
            DayOfYear::Cyprès => "Cyprès",
            DayOfYear::Lierre => "Lierre",
            DayOfYear::Sabine => "Sabine",
            DayOfYear::Hoyau => "Hoyau",
            DayOfYear::ÉrableÀSucre => "Érable à sucre",
            DayOfYear::Bruyère => "Bruyère",
            DayOfYear::Roseau => "Roseau",
            DayOfYear::Oseille => "Oseille",
            DayOfYear::Grillon => "Grillon",
            DayOfYear::Pignon => "Pignon",
            DayOfYear::Liège => "Liège",
            DayOfYear::Truffe => "Truffe",
            DayOfYear::Olive => "Olive",
            DayOfYear::Pelle => "Pelle",
            DayOfYear::Tourbe => "Tourbe",
            DayOfYear::Houille => "Houille",
            DayOfYear::Bitume => "Bitume",
            DayOfYear::Soufre => "Soufre",
            DayOfYear::Chien => "Chien",
            DayOfYear::Lave => "Lave",
            DayOfYear::TerreVégétale => "Terre végétale",
            DayOfYear::Fumier => "Fumier",
            DayOfYear::Salpêtre => "Salpêtre",
            DayOfYear::Fléau => "Fléau",
            DayOfYear::Granit => "Granit",
            DayOfYear::Argile => "Argile",
            DayOfYear::Ardoise => "Ardoise",
            DayOfYear::Grès => "Grès",
            DayOfYear::Lapin => "Lapin",
            DayOfYear::Silex => "Silex",
            DayOfYear::Marne => "Marne",
            DayOfYear::PierreÀChaux => "Pierre à chaux",
            DayOfYear::Marbre => "Marbre",
            DayOfYear::Van => "Van",
            DayOfYear::PierreÀPlâtre => "Pierre à plâtre",
            DayOfYear::Sel => "Sel",
            DayOfYear::Fer => "Fer",
            DayOfYear::Cuivre => "Cuivre",
            DayOfYear::Chat => "Chat",
            DayOfYear::Étain => "Étain",
            DayOfYear::Plomb => "Plomb",
            DayOfYear::Zinc => "Zinc",
            DayOfYear::Mercure => "Mercure",
            DayOfYear::Crible => "Crible",
            DayOfYear::Lauréole => "Lauréole",
            DayOfYear::Mousse => "Mousse",
            DayOfYear::Fragon => "Fragon",
            DayOfYear::PerceNeige => "Perce-neige",
            DayOfYear::Taureau => "Taureau",
            DayOfYear::LaurierThym => "Laurier-thym",
            DayOfYear::Amadouvier => "Amadouvier",
            DayOfYear::Mézéréon => "Mézéréon",
            DayOfYear::Peuplier => "Peuplier",
            DayOfYear::Coignée => "Coignée",
            DayOfYear::Ellébore => "Ellébore",
            DayOfYear::Brocoli => "Brocoli",
            DayOfYear::Laurier => "Laurier",
            DayOfYear::Avelinier => "Avelinier",
            DayOfYear::Vache => "Vache",
            DayOfYear::Buis => "Buis",
            DayOfYear::Lichen => "Lichen",
            DayOfYear::If => "If",
            DayOfYear::Pulmonaire => "Pulmonaire",
            DayOfYear::Serpette => "Serpette",
            DayOfYear::Thlaspi => "Thlaspi",
            DayOfYear::Thimelé => "Thimelé",
            DayOfYear::Chiendent => "Chiendent",
            DayOfYear::Trainasse => "Trainasse",
            DayOfYear::Lièvre => "Lièvre",
            DayOfYear::Guède => "Guède",
            DayOfYear::Noisetier => "Noisetier",
            DayOfYear::Cyclamen => "Cyclamen",
            DayOfYear::Chélidoine => "Chélidoine",
            DayOfYear::Traîneau => "Traîneau",
            DayOfYear::Tussilage => "Tussilage",
            DayOfYear::Cornouiller => "Cornouiller",
            DayOfYear::Violier => "Violier",
            DayOfYear::Troène => "Troène",
            DayOfYear::Bouc => "Bouc",
            DayOfYear::Asaret => "Asaret",
            DayOfYear::Alaterne => "Alaterne",
            DayOfYear::Violette => "Violette",
            DayOfYear::Marceau => "Marceau",
            DayOfYear::Bêche => "Bêche",
            DayOfYear::Narcisse => "Narcisse",
            DayOfYear::Orme => "Orme",
            DayOfYear::Fumeterre => "Fumeterre",
            DayOfYear::Vélar => "Vélar",
            DayOfYear::Chèvre => "Chèvre",
            DayOfYear::Épinard => "Épinard",
            DayOfYear::Doronic => "Doronic",
            DayOfYear::Mouron => "Mouron",
            DayOfYear::Cerfeuil => "Cerfeuil",
            DayOfYear::Cordeau => "Cordeau",
            DayOfYear::Mandragore => "Mandragore",
            DayOfYear::Persil => "Persil",
            DayOfYear::Cochléaria => "Cochléaria",
            DayOfYear::Pâquerette => "Pâquerette",
            DayOfYear::Thon => "Thon",
            DayOfYear::Pissenlit => "Pissenlit",
            DayOfYear::Sylvie => "Sylvie",
            DayOfYear::Capillaire => "Capillaire",
            DayOfYear::Frêne => "Frêne",
            DayOfYear::Plantoir => "Plantoir",
            DayOfYear::Primevère => "Primevère",
            DayOfYear::Platane => "Platane",
            DayOfYear::Asperge => "Asperge",
            DayOfYear::Tulipe => "Tulipe",
            DayOfYear::Poule => "Poule",
            DayOfYear::Bette => "Bette",
            DayOfYear::Bouleau => "Bouleau",
            DayOfYear::Jonquille => "Jonquille",
            DayOfYear::Aulne => "Aulne",
            DayOfYear::Couvoir => "Couvoir",
            DayOfYear::Pervenche => "Pervenche",
            DayOfYear::Charme => "Charme",
            DayOfYear::Morille => "Morille",
            DayOfYear::Hêtre => "Hêtre",
            DayOfYear::Abeille => "Abeille",
            DayOfYear::Laitue => "Laitue",
            DayOfYear::Mélèze => "Mélèze",
            DayOfYear::Ciguë => "Ciguë",
            DayOfYear::Radis => "Radis",
            DayOfYear::Ruche => "Ruche",
            DayOfYear::Gainier => "Gainier",
            DayOfYear::Romaine => "Romaine",
            DayOfYear::Marronnier => "Marronnier",
            DayOfYear::Roquette => "Roquette",
            DayOfYear::Pigeon => "Pigeon",
            DayOfYear::Lilas => "Lilas",
            DayOfYear::Anémone => "Anémone",
            DayOfYear::Pensée => "Pensée",
            DayOfYear::Myrtille => "Myrtille",
            DayOfYear::Greffoir => "Greffoir",
            DayOfYear::Rose => "Rose",
            DayOfYear::Chêne => "Chêne",
            DayOfYear::Fougère => "Fougère",
            DayOfYear::Aubépine => "Aubépine",
            DayOfYear::Rossignol => "Rossignol",
            DayOfYear::Ancolie => "Ancolie",
            DayOfYear::Muguet => "Muguet",
            DayOfYear::Champignon => "Champignon",
            DayOfYear::Hyacinthe => "Hyacinthe",
            DayOfYear::Râteau => "Râteau",
            DayOfYear::Rhubarbe => "Rhubarbe",
            DayOfYear::Sainfoin => "Sainfoin",
            DayOfYear::BâtonDOr => "Bâton d'or",
            DayOfYear::Chamerisier => "Chamerisier",
            DayOfYear::VerÀSoie => "Ver à soie",
            DayOfYear::Consoude => "Consoude",
            DayOfYear::Pimprenelle => "Pimprenelle",
            DayOfYear::CorbeilleDOr => "Corbeille d'or",
            DayOfYear::Arroche => "Arroche",
            DayOfYear::Sarcloir => "Sarcloir",
            DayOfYear::Statice => "Statice",
            DayOfYear::Fritillaire => "Fritillaire",
            DayOfYear::Bourrache => "Bourrache",
            DayOfYear::Valériane => "Valériane",
            DayOfYear::Carpe => "Carpe",
            DayOfYear::Fusain => "Fusain",
            DayOfYear::Civette => "Civette",
            DayOfYear::Buglosse => "Buglosse",
            DayOfYear::Sénevé => "Sénevé",
            DayOfYear::Houlette => "Houlette",
            DayOfYear::Luzerne => "Luzerne",
            DayOfYear::Hémérocalle => "Hémérocalle",
            DayOfYear::Trèfle => "Trèfle",
            DayOfYear::Angélique => "Angélique",
            DayOfYear::Canard => "Canard",
            DayOfYear::Mélisse => "Mélisse",
            DayOfYear::Fromental => "Fromental",
            DayOfYear::Martagon => "Martagon",
            DayOfYear::Serpolet => "Serpolet",
            DayOfYear::Faux => "Faux",
            DayOfYear::Fraise => "Fraise",
            DayOfYear::Bétoine => "Bétoine",
            DayOfYear::Pois => "Pois",
            DayOfYear::Acacia => "Acacia",
            DayOfYear::Caille => "Caille",
            DayOfYear::Œillet => "Œillet",
            DayOfYear::Sureau => "Sureau",
            DayOfYear::Pavot => "Pavot",
            DayOfYear::Tilleul => "Tilleul",
            DayOfYear::Fourche => "Fourche",
            DayOfYear::Barbeau => "Barbeau",
            DayOfYear::Camomille => "Camomille",
            DayOfYear::Chèvrefeuille => "Chèvrefeuille",
            DayOfYear::CailleLait => "Caille-lait",
            DayOfYear::Tanche => "Tanche",
            DayOfYear::Jasmin => "Jasmin",
            DayOfYear::Verveine => "Verveine",
            DayOfYear::Thym => "Thym",
            DayOfYear::Pivoine => "Pivoine",
            DayOfYear::Chariot => "Chariot",
            DayOfYear::Seigle => "Seigle",
            DayOfYear::Avoine => "Avoine",
            DayOfYear::Oignon => "Oignon",
            DayOfYear::Véronique => "Véronique",
            DayOfYear::Mulet => "Mulet",
            DayOfYear::Romarin => "Romarin",
            DayOfYear::Concombre => "Concombre",
            DayOfYear::Échalote => "Échalote",
            DayOfYear::Absinthe => "Absinthe",
            DayOfYear::Faucille => "Faucille",
            DayOfYear::Coriandre => "Coriandre",
            DayOfYear::Artichaut => "Artichaut",
            DayOfYear::Girofle => "Girofle",
            DayOfYear::Lavande => "Lavande",
            DayOfYear::Chamois => "Chamois",
            DayOfYear::Tabac => "Tabac",
            DayOfYear::Groseille => "Groseille",
            DayOfYear::Gesse => "Gesse",
            DayOfYear::Cerise => "Cerise",
            DayOfYear::Parc => "Parc",
            DayOfYear::Menthe => "Menthe",
            DayOfYear::Cumin => "Cumin",
            DayOfYear::Haricot => "Haricot",
            DayOfYear::Orcanète => "Orcanète",
            DayOfYear::Pintade => "Pintade",
            DayOfYear::Sauge => "Sauge",
            DayOfYear::Ail => "Ail",
            DayOfYear::Vesce => "Vesce",
            DayOfYear::Blé => "Blé",
            DayOfYear::Chalémie => "Chalémie",
            DayOfYear::Épeautre => "Épeautre",
            DayOfYear::BouillonBlanc => "Bouillon blanc",
            DayOfYear::Melon => "Melon",
            DayOfYear::Ivraie => "Ivraie",
            DayOfYear::Bélier => "Bélier",
            DayOfYear::Prêle => "Prêle",
            DayOfYear::Armoise => "Armoise",
            DayOfYear::Carthame => "Carthame",
            DayOfYear::Mûre => "Mûre",
            DayOfYear::Arrosoir => "Arrosoir",
            DayOfYear::Panic => "Panic",
            DayOfYear::Salicorne => "Salicorne",
            DayOfYear::Abricot => "Abricot",
            DayOfYear::Basilic => "Basilic",
            DayOfYear::Brebis => "Brebis",
            DayOfYear::Guimauve => "Guimauve",
            DayOfYear::Lin => "Lin",
            DayOfYear::Amande => "Amande",
            DayOfYear::Gentiane => "Gentiane",
            DayOfYear::Écluse => "Écluse",
            DayOfYear::Carline => "Carline",
            DayOfYear::Câprier => "Câprier",
            DayOfYear::Lentille => "Lentille",
            DayOfYear::Aunée => "Aunée",
            DayOfYear::Loutre => "Loutre",
            DayOfYear::Myrte => "Myrte",
            DayOfYear::Colza => "Colza",
            DayOfYear::Lupin => "Lupin",
            DayOfYear::Coton => "Coton",
            DayOfYear::Moulin => "Moulin",
            DayOfYear::Prune => "Prune",
            DayOfYear::Millet => "Millet",
            DayOfYear::Lycoperdon => "Lycoperdon",
            DayOfYear::Escourgeon => "Escourgeon",
            DayOfYear::Saumon => "Saumon",
            DayOfYear::Tubéreuse => "Tubéreuse",
            DayOfYear::Sucrion => "Sucrion",
            DayOfYear::Apocyn => "Apocyn",
            DayOfYear::Réglisse => "Réglisse",
            DayOfYear::Échelle => "Échelle",
            DayOfYear::Pastèque => "Pastèque",
            DayOfYear::Fenouil => "Fenouil",
            DayOfYear::ÉpineVinette => "Épine vinette",
            DayOfYear::Noix => "Noix",
            DayOfYear::Truite => "Truite",
            DayOfYear::Citron => "Citron",
            DayOfYear::Cardère => "Cardère",
            DayOfYear::Nerprun => "Nerprun",
            DayOfYear::Tagette => "Tagette",
            DayOfYear::Hotte => "Hotte",
            DayOfYear::Églantier => "Églantier",
            DayOfYear::Noisette => "Noisette",
            DayOfYear::Houblon => "Houblon",
            DayOfYear::Sorgho => "Sorgho",
            DayOfYear::Écrevisse => "Écrevisse",
            DayOfYear::Bigarade => "Bigarade",
            DayOfYear::VergeDOr => "Verge d'or",
            DayOfYear::Maïs => "Maïs",
            DayOfYear::Marron => "Marron",
            DayOfYear::Panier => "Panier",
            DayOfYear::LaFêteDeLaVertu => "La fête de la vertu",
            DayOfYear::LaFêteDuGénie => "La fête du génie",
            DayOfYear::LaFêteDuTravail => "La fête du travail",
            DayOfYear::LaFêteDeLOpinion => "La fête de l'opinion",
            DayOfYear::LaFêteDesRécompenses => "La fête des récompenses",
            DayOfYear::LaFêteDeLaRévolution => "La fête de la Révolution",
        };
        write!(f, "{day_str}")
    }
}

impl TryFrom<u32> for DayOfYear {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DayOfYear::Raisin),
            2 => Ok(DayOfYear::Safran),
            3 => Ok(DayOfYear::Châtaigne),
            4 => Ok(DayOfYear::Colchique),
            5 => Ok(DayOfYear::Cheval),
            6 => Ok(DayOfYear::Balsamine),
            7 => Ok(DayOfYear::Carotte),
            8 => Ok(DayOfYear::Amaranthe),
            9 => Ok(DayOfYear::Panais),
            10 => Ok(DayOfYear::Cuve),
            11 => Ok(DayOfYear::PommeDeTerre),
            12 => Ok(DayOfYear::Immortelle),
            13 => Ok(DayOfYear::Potiron),
            14 => Ok(DayOfYear::Réséda),
            15 => Ok(DayOfYear::Âne),
            16 => Ok(DayOfYear::BelleDeNuit),
            17 => Ok(DayOfYear::Citrouille),
            18 => Ok(DayOfYear::Sarrasin),
            19 => Ok(DayOfYear::Tournesol),
            20 => Ok(DayOfYear::Pressoir),
            21 => Ok(DayOfYear::Chanvre),
            22 => Ok(DayOfYear::Pêche),
            23 => Ok(DayOfYear::Navet),
            24 => Ok(DayOfYear::Amaryllis),
            25 => Ok(DayOfYear::Bœuf),
            26 => Ok(DayOfYear::Aubergine),
            27 => Ok(DayOfYear::Piment),
            28 => Ok(DayOfYear::Tomate),
            29 => Ok(DayOfYear::Orge),
            30 => Ok(DayOfYear::Tonneau),
            31 => Ok(DayOfYear::Pomme),
            32 => Ok(DayOfYear::Céleri),
            33 => Ok(DayOfYear::Poire),
            34 => Ok(DayOfYear::Betterave),
            35 => Ok(DayOfYear::Oie),
            36 => Ok(DayOfYear::Héliotrope),
            37 => Ok(DayOfYear::Figue),
            38 => Ok(DayOfYear::Scorsonère),
            39 => Ok(DayOfYear::Alisier),
            40 => Ok(DayOfYear::Charrue),
            41 => Ok(DayOfYear::Salsifis),
            42 => Ok(DayOfYear::Mâcre),
            43 => Ok(DayOfYear::Topinambour),
            44 => Ok(DayOfYear::Endive),
            45 => Ok(DayOfYear::Dindon),
            46 => Ok(DayOfYear::Chervis),
            47 => Ok(DayOfYear::Cresson),
            48 => Ok(DayOfYear::Dentelaire),
            49 => Ok(DayOfYear::Grenade),
            50 => Ok(DayOfYear::Herse),
            51 => Ok(DayOfYear::Bacchante),
            52 => Ok(DayOfYear::Azerole),
            53 => Ok(DayOfYear::Garance),
            54 => Ok(DayOfYear::Orange),
            55 => Ok(DayOfYear::Faisan),
            56 => Ok(DayOfYear::Pistache),
            57 => Ok(DayOfYear::Macjonc),
            58 => Ok(DayOfYear::Coing),
            59 => Ok(DayOfYear::Cormier),
            60 => Ok(DayOfYear::Rouleau),
            61 => Ok(DayOfYear::Raiponce),
            62 => Ok(DayOfYear::Turneps),
            63 => Ok(DayOfYear::Chicorée),
            64 => Ok(DayOfYear::Nèfle),
            65 => Ok(DayOfYear::Cochon),
            66 => Ok(DayOfYear::Mâche),
            67 => Ok(DayOfYear::ChouFleur),
            68 => Ok(DayOfYear::Miel),
            69 => Ok(DayOfYear::Genièvre),
            70 => Ok(DayOfYear::Pioche),
            71 => Ok(DayOfYear::Cire),
            72 => Ok(DayOfYear::Raifort),
            73 => Ok(DayOfYear::Cèdre),
            74 => Ok(DayOfYear::Sapin),
            75 => Ok(DayOfYear::Chevreuil),
            76 => Ok(DayOfYear::Ajonc),
            77 => Ok(DayOfYear::Cyprès),
            78 => Ok(DayOfYear::Lierre),
            79 => Ok(DayOfYear::Sabine),
            80 => Ok(DayOfYear::Hoyau),
            81 => Ok(DayOfYear::ÉrableÀSucre),
            82 => Ok(DayOfYear::Bruyère),
            83 => Ok(DayOfYear::Roseau),
            84 => Ok(DayOfYear::Oseille),
            85 => Ok(DayOfYear::Grillon),
            86 => Ok(DayOfYear::Pignon),
            87 => Ok(DayOfYear::Liège),
            88 => Ok(DayOfYear::Truffe),
            89 => Ok(DayOfYear::Olive),
            90 => Ok(DayOfYear::Pelle),
            91 => Ok(DayOfYear::Tourbe),
            92 => Ok(DayOfYear::Houille),
            93 => Ok(DayOfYear::Bitume),
            94 => Ok(DayOfYear::Soufre),
            95 => Ok(DayOfYear::Chien),
            96 => Ok(DayOfYear::Lave),
            97 => Ok(DayOfYear::TerreVégétale),
            98 => Ok(DayOfYear::Fumier),
            99 => Ok(DayOfYear::Salpêtre),
            100 => Ok(DayOfYear::Fléau),
            101 => Ok(DayOfYear::Granit),
            102 => Ok(DayOfYear::Argile),
            103 => Ok(DayOfYear::Ardoise),
            104 => Ok(DayOfYear::Grès),
            105 => Ok(DayOfYear::Lapin),
            106 => Ok(DayOfYear::Silex),
            107 => Ok(DayOfYear::Marne),
            108 => Ok(DayOfYear::PierreÀChaux),
            109 => Ok(DayOfYear::Marbre),
            110 => Ok(DayOfYear::Van),
            111 => Ok(DayOfYear::PierreÀPlâtre),
            112 => Ok(DayOfYear::Sel),
            113 => Ok(DayOfYear::Fer),
            114 => Ok(DayOfYear::Cuivre),
            115 => Ok(DayOfYear::Chat),
            116 => Ok(DayOfYear::Étain),
            117 => Ok(DayOfYear::Plomb),
            118 => Ok(DayOfYear::Zinc),
            119 => Ok(DayOfYear::Mercure),
            120 => Ok(DayOfYear::Crible),
            121 => Ok(DayOfYear::Lauréole),
            122 => Ok(DayOfYear::Mousse),
            123 => Ok(DayOfYear::Fragon),
            124 => Ok(DayOfYear::PerceNeige),
            125 => Ok(DayOfYear::Taureau),
            126 => Ok(DayOfYear::LaurierThym),
            127 => Ok(DayOfYear::Amadouvier),
            128 => Ok(DayOfYear::Mézéréon),
            129 => Ok(DayOfYear::Peuplier),
            130 => Ok(DayOfYear::Coignée),
            131 => Ok(DayOfYear::Ellébore),
            132 => Ok(DayOfYear::Brocoli),
            133 => Ok(DayOfYear::Laurier),
            134 => Ok(DayOfYear::Avelinier),
            135 => Ok(DayOfYear::Vache),
            136 => Ok(DayOfYear::Buis),
            137 => Ok(DayOfYear::Lichen),
            138 => Ok(DayOfYear::If),
            139 => Ok(DayOfYear::Pulmonaire),
            140 => Ok(DayOfYear::Serpette),
            141 => Ok(DayOfYear::Thlaspi),
            142 => Ok(DayOfYear::Thimelé),
            143 => Ok(DayOfYear::Chiendent),
            144 => Ok(DayOfYear::Trainasse),
            145 => Ok(DayOfYear::Lièvre),
            146 => Ok(DayOfYear::Guède),
            147 => Ok(DayOfYear::Noisetier),
            148 => Ok(DayOfYear::Cyclamen),
            149 => Ok(DayOfYear::Chélidoine),
            150 => Ok(DayOfYear::Traîneau),
            151 => Ok(DayOfYear::Tussilage),
            152 => Ok(DayOfYear::Cornouiller),
            153 => Ok(DayOfYear::Violier),
            154 => Ok(DayOfYear::Troène),
            155 => Ok(DayOfYear::Bouc),
            156 => Ok(DayOfYear::Asaret),
            157 => Ok(DayOfYear::Alaterne),
            158 => Ok(DayOfYear::Violette),
            159 => Ok(DayOfYear::Marceau),
            160 => Ok(DayOfYear::Bêche),
            161 => Ok(DayOfYear::Narcisse),
            162 => Ok(DayOfYear::Orme),
            163 => Ok(DayOfYear::Fumeterre),
            164 => Ok(DayOfYear::Vélar),
            165 => Ok(DayOfYear::Chèvre),
            166 => Ok(DayOfYear::Épinard),
            167 => Ok(DayOfYear::Doronic),
            168 => Ok(DayOfYear::Mouron),
            169 => Ok(DayOfYear::Cerfeuil),
            170 => Ok(DayOfYear::Cordeau),
            171 => Ok(DayOfYear::Mandragore),
            172 => Ok(DayOfYear::Persil),
            173 => Ok(DayOfYear::Cochléaria),
            174 => Ok(DayOfYear::Pâquerette),
            175 => Ok(DayOfYear::Thon),
            176 => Ok(DayOfYear::Pissenlit),
            177 => Ok(DayOfYear::Sylvie),
            178 => Ok(DayOfYear::Capillaire),
            179 => Ok(DayOfYear::Frêne),
            180 => Ok(DayOfYear::Plantoir),
            181 => Ok(DayOfYear::Primevère),
            182 => Ok(DayOfYear::Platane),
            183 => Ok(DayOfYear::Asperge),
            184 => Ok(DayOfYear::Tulipe),
            185 => Ok(DayOfYear::Poule),
            186 => Ok(DayOfYear::Bette),
            187 => Ok(DayOfYear::Bouleau),
            188 => Ok(DayOfYear::Jonquille),
            189 => Ok(DayOfYear::Aulne),
            190 => Ok(DayOfYear::Couvoir),
            191 => Ok(DayOfYear::Pervenche),
            192 => Ok(DayOfYear::Charme),
            193 => Ok(DayOfYear::Morille),
            194 => Ok(DayOfYear::Hêtre),
            195 => Ok(DayOfYear::Abeille),
            196 => Ok(DayOfYear::Laitue),
            197 => Ok(DayOfYear::Mélèze),
            198 => Ok(DayOfYear::Ciguë),
            199 => Ok(DayOfYear::Radis),
            200 => Ok(DayOfYear::Ruche),
            201 => Ok(DayOfYear::Gainier),
            202 => Ok(DayOfYear::Romaine),
            203 => Ok(DayOfYear::Marronnier),
            204 => Ok(DayOfYear::Roquette),
            205 => Ok(DayOfYear::Pigeon),
            206 => Ok(DayOfYear::Lilas),
            207 => Ok(DayOfYear::Anémone),
            208 => Ok(DayOfYear::Pensée),
            209 => Ok(DayOfYear::Myrtille),
            210 => Ok(DayOfYear::Greffoir),
            211 => Ok(DayOfYear::Rose),
            212 => Ok(DayOfYear::Chêne),
            213 => Ok(DayOfYear::Fougère),
            214 => Ok(DayOfYear::Aubépine),
            215 => Ok(DayOfYear::Rossignol),
            216 => Ok(DayOfYear::Ancolie),
            217 => Ok(DayOfYear::Muguet),
            218 => Ok(DayOfYear::Champignon),
            219 => Ok(DayOfYear::Hyacinthe),
            220 => Ok(DayOfYear::Râteau),
            221 => Ok(DayOfYear::Rhubarbe),
            222 => Ok(DayOfYear::Sainfoin),
            223 => Ok(DayOfYear::BâtonDOr),
            224 => Ok(DayOfYear::Chamerisier),
            225 => Ok(DayOfYear::VerÀSoie),
            226 => Ok(DayOfYear::Consoude),
            227 => Ok(DayOfYear::Pimprenelle),
            228 => Ok(DayOfYear::CorbeilleDOr),
            229 => Ok(DayOfYear::Arroche),
            230 => Ok(DayOfYear::Sarcloir),
            231 => Ok(DayOfYear::Statice),
            232 => Ok(DayOfYear::Fritillaire),
            233 => Ok(DayOfYear::Bourrache),
            234 => Ok(DayOfYear::Valériane),
            235 => Ok(DayOfYear::Carpe),
            236 => Ok(DayOfYear::Fusain),
            237 => Ok(DayOfYear::Civette),
            238 => Ok(DayOfYear::Buglosse),
            239 => Ok(DayOfYear::Sénevé),
            240 => Ok(DayOfYear::Houlette),
            241 => Ok(DayOfYear::Luzerne),
            242 => Ok(DayOfYear::Hémérocalle),
            243 => Ok(DayOfYear::Trèfle),
            244 => Ok(DayOfYear::Angélique),
            245 => Ok(DayOfYear::Canard),
            246 => Ok(DayOfYear::Mélisse),
            247 => Ok(DayOfYear::Fromental),
            248 => Ok(DayOfYear::Martagon),
            249 => Ok(DayOfYear::Serpolet),
            250 => Ok(DayOfYear::Faux),
            251 => Ok(DayOfYear::Fraise),
            252 => Ok(DayOfYear::Bétoine),
            253 => Ok(DayOfYear::Pois),
            254 => Ok(DayOfYear::Acacia),
            255 => Ok(DayOfYear::Caille),
            256 => Ok(DayOfYear::Œillet),
            257 => Ok(DayOfYear::Sureau),
            258 => Ok(DayOfYear::Pavot),
            259 => Ok(DayOfYear::Tilleul),
            260 => Ok(DayOfYear::Fourche),
            261 => Ok(DayOfYear::Barbeau),
            262 => Ok(DayOfYear::Camomille),
            263 => Ok(DayOfYear::Chèvrefeuille),
            264 => Ok(DayOfYear::CailleLait),
            265 => Ok(DayOfYear::Tanche),
            266 => Ok(DayOfYear::Jasmin),
            267 => Ok(DayOfYear::Verveine),
            268 => Ok(DayOfYear::Thym),
            269 => Ok(DayOfYear::Pivoine),
            270 => Ok(DayOfYear::Chariot),
            271 => Ok(DayOfYear::Seigle),
            272 => Ok(DayOfYear::Avoine),
            273 => Ok(DayOfYear::Oignon),
            274 => Ok(DayOfYear::Véronique),
            275 => Ok(DayOfYear::Mulet),
            276 => Ok(DayOfYear::Romarin),
            277 => Ok(DayOfYear::Concombre),
            278 => Ok(DayOfYear::Échalote),
            279 => Ok(DayOfYear::Absinthe),
            280 => Ok(DayOfYear::Faucille),
            281 => Ok(DayOfYear::Coriandre),
            282 => Ok(DayOfYear::Artichaut),
            283 => Ok(DayOfYear::Girofle),
            284 => Ok(DayOfYear::Lavande),
            285 => Ok(DayOfYear::Chamois),
            286 => Ok(DayOfYear::Tabac),
            287 => Ok(DayOfYear::Groseille),
            288 => Ok(DayOfYear::Gesse),
            289 => Ok(DayOfYear::Cerise),
            290 => Ok(DayOfYear::Parc),
            291 => Ok(DayOfYear::Menthe),
            292 => Ok(DayOfYear::Cumin),
            293 => Ok(DayOfYear::Haricot),
            294 => Ok(DayOfYear::Orcanète),
            295 => Ok(DayOfYear::Pintade),
            296 => Ok(DayOfYear::Sauge),
            297 => Ok(DayOfYear::Ail),
            298 => Ok(DayOfYear::Vesce),
            299 => Ok(DayOfYear::Blé),
            300 => Ok(DayOfYear::Chalémie),
            301 => Ok(DayOfYear::Épeautre),
            302 => Ok(DayOfYear::BouillonBlanc),
            303 => Ok(DayOfYear::Melon),
            304 => Ok(DayOfYear::Ivraie),
            305 => Ok(DayOfYear::Bélier),
            306 => Ok(DayOfYear::Prêle),
            307 => Ok(DayOfYear::Armoise),
            308 => Ok(DayOfYear::Carthame),
            309 => Ok(DayOfYear::Mûre),
            310 => Ok(DayOfYear::Arrosoir),
            311 => Ok(DayOfYear::Panic),
            312 => Ok(DayOfYear::Salicorne),
            313 => Ok(DayOfYear::Abricot),
            314 => Ok(DayOfYear::Basilic),
            315 => Ok(DayOfYear::Brebis),
            316 => Ok(DayOfYear::Guimauve),
            317 => Ok(DayOfYear::Lin),
            318 => Ok(DayOfYear::Amande),
            319 => Ok(DayOfYear::Gentiane),
            320 => Ok(DayOfYear::Écluse),
            321 => Ok(DayOfYear::Carline),
            322 => Ok(DayOfYear::Câprier),
            323 => Ok(DayOfYear::Lentille),
            324 => Ok(DayOfYear::Aunée),
            325 => Ok(DayOfYear::Loutre),
            326 => Ok(DayOfYear::Myrte),
            327 => Ok(DayOfYear::Colza),
            328 => Ok(DayOfYear::Lupin),
            329 => Ok(DayOfYear::Coton),
            330 => Ok(DayOfYear::Moulin),
            331 => Ok(DayOfYear::Prune),
            332 => Ok(DayOfYear::Millet),
            333 => Ok(DayOfYear::Lycoperdon),
            334 => Ok(DayOfYear::Escourgeon),
            335 => Ok(DayOfYear::Saumon),
            336 => Ok(DayOfYear::Tubéreuse),
            337 => Ok(DayOfYear::Sucrion),
            338 => Ok(DayOfYear::Apocyn),
            339 => Ok(DayOfYear::Réglisse),
            340 => Ok(DayOfYear::Échelle),
            341 => Ok(DayOfYear::Pastèque),
            342 => Ok(DayOfYear::Fenouil),
            343 => Ok(DayOfYear::ÉpineVinette),
            344 => Ok(DayOfYear::Noix),
            345 => Ok(DayOfYear::Truite),
            346 => Ok(DayOfYear::Citron),
            347 => Ok(DayOfYear::Cardère),
            348 => Ok(DayOfYear::Nerprun),
            349 => Ok(DayOfYear::Tagette),
            350 => Ok(DayOfYear::Hotte),
            351 => Ok(DayOfYear::Églantier),
            352 => Ok(DayOfYear::Noisette),
            353 => Ok(DayOfYear::Houblon),
            354 => Ok(DayOfYear::Sorgho),
            355 => Ok(DayOfYear::Écrevisse),
            356 => Ok(DayOfYear::Bigarade),
            357 => Ok(DayOfYear::VergeDOr),
            358 => Ok(DayOfYear::Maïs),
            359 => Ok(DayOfYear::Marron),
            360 => Ok(DayOfYear::Panier),
            361 => Ok(DayOfYear::LaFêteDeLaVertu),
            362 => Ok(DayOfYear::LaFêteDuGénie),
            363 => Ok(DayOfYear::LaFêteDuTravail),
            364 => Ok(DayOfYear::LaFêteDeLOpinion),
            365 => Ok(DayOfYear::LaFêteDesRécompenses),
            366 => Ok(DayOfYear::LaFêteDeLaRévolution),
            _ => Err(()),
        }
    }
}

impl DayOfYear {
    /// Create a `DayOfYear` from a month (1-13) and day (1-30 for months 1-12, 1-6 for month 13).
    pub fn try_from_month_day(month: impl TryInto<Month>, day: Day) -> Option<Self> {
        match month.try_into().ok()? {
            Month::SansCulottides => {
                if day > 0 && day <= DAYS_IN_SANS_CULOTTIDES_LEAP_YEAR {
                    DayOfYear::try_from(DAYS_IN_NORMAL_MONTHS + day).ok()
                } else {
                    None
                }
            }
            m if day > 0 && day <= DAYS_IN_MONTH => {
                DayOfYear::try_from((m as u32 - 1) * DAYS_IN_MONTH + day).ok()
            }
            _ => None,
        }
    }

    /// Convert a `DayOfYear` into a (month, day) tuple.
    pub fn into_month_day(self) -> (Month, Day) {
        let day_of_year = self as u32;
        if day_of_year > DAYS_IN_NORMAL_MONTHS {
            (Month::SansCulottides, day_of_year - DAYS_IN_NORMAL_MONTHS)
        } else {
            let month = (day_of_year - 1) / DAYS_IN_MONTH + 1;
            let day = (day_of_year - 1) % DAYS_IN_MONTH + 1;
            (Month::try_from(month).unwrap(), day)
        }
    }

    /// Returns `true` if the day is in the Sans-culottides.
    pub fn is_sans_culottides(self) -> bool {
        self as u32 > DAYS_IN_NORMAL_MONTHS
    }

    /// Returns `true` if the day is the leap day (Fête de la Révolution).
    pub fn is_leap_day(self) -> bool {
        matches!(self, DayOfYear::LaFêteDeLaRévolution)
    }

    /// Returns `true` if the day is legal for the given year in the given leap year system.
    pub fn is_legal_for_year<L: LeapSystem>(self, year: Year) -> bool {
        !self.is_leap_day() || L::is_leap_year(year)
    }
}

#[cfg(test)]
mod test {
    use crate::date::leap::DelambreRomme;

    use super::*;

    #[test]
    fn test_sans_culottides_check() {
        for day in DayOfYear::enumerator() {
            assert_eq!(
                day.is_sans_culottides(),
                matches!(day.into_month_day(), (Month::SansCulottides, _))
            );
        }
    }

    #[test]
    fn test_leap_day_check() {
        let leap_day = DayOfYear::LaFêteDeLaRévolution;
        assert!(leap_day.is_leap_day());

        assert!(leap_day.is_legal_for_year::<DelambreRomme>(3));
        assert!(!leap_day.is_legal_for_year::<DelambreRomme>(4));
    }

    #[test]
    fn from_month_day() {
        for doy in DayOfYear::enumerator() {
            let (month, day) = doy.into_month_day();
            assert_eq!(Some(doy), DayOfYear::try_from_month_day(month, day));
        }
    }
}
