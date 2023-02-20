fetch("../www/html/components/index-nav.html")
    .then(response => {
        return response.text();
    }).then(data => {
        document.querySelector(".nav-container").innerHTML = data;
    });