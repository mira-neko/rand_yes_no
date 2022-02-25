async function fetchAsync(url) {
    let response = await fetch(url);
    let data = await response.json();
    return data;
  }

jQuery(document).ready(function () {
    $.get("http://192.168.0.174:8080/rnd", function(data) {
        $("p").text(data);
    });
});
