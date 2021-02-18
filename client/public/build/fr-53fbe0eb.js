
(function(l, r) { if (l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (window.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(window.document);
'use strict';

require('./main.js');

var home = {
	welcome: "Bienvenu sur Peerchat ! Ce site vous permet d'ajouter un chat en temps réel à n'importe quelle vidéo Peertube. Pour cela, vous pouvez entrer l'URL (l'adresse à prendre de la barre d'adresse) dans le champ ci-dessous, vous choisir un pseudo, et commencer à papoter :-)",
	peertube_url: "URL Peertube :",
	nickname: "Votre surnom :"
};
var fr = {
	"home-link": "Accueil",
	home: home
};

exports.default = fr;
exports.home = home;
//# sourceMappingURL=fr-53fbe0eb.js.map
