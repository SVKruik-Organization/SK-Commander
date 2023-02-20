fetch("../html/components/pages-nav.html")
    .then(response => {
        return response.text();
    }).then(data => {
        document.querySelector(".nav-container").innerHTML = data;
    });