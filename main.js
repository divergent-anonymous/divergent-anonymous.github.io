// function get_payload(call, stri) {
//   console.log("items is s ", call)
//   // var items = call.clipboardData.items;
//
//   if(items == undefined){
//     if(typeof(callback) == "function"){
//       callback(undefined);
//     }
//   };
//
//   for (var i = 0; i < items.length; i++) {
//     // Skip content if not image
//     if (items[i].type.indexOf("image") == -1) continue;
//     // Retrieve image on clipboard as blob
//     var blob = items[i].getAsFile();
//
//     console.log(blob)
//     if(typeof(callback) == "function"){
//       callback(blob);
//     }
//   }
//
//   console.log(call);
// }

function get_payloada(call, stri) {
  navigator.permissions.query({name: "clipboard-read"}).then(result => {
    // If permission to read the clipboard is granted or if the user will
    // be prompted to allow it, we proceed.

    if (result.state == "granted" || result.state == "prompt") {
      navigator.clipboard.read().then(data => {
	console.log(data)
	for (let i=0; i<data.items.length; i++) {
	  if (data.items[i].type != "image/png") {
	    alert("Clipboard contains non-image data. Unable to access it.");
	  } else {
	    const blob = data.items[i].getType("image/png");
	    imgElem.src = URL.createObjectURL(blob);
	  }
	}
      });
    }
  });

}



async function get_payload(call, asdf) {
  navigator.clipboard.writeText(asdf).then(function() {
    console.log('Async: Copying to clipboard was successful!');
  }, function(err) {
    console.error('Async: Could not copy text: ', err);
  });
}


