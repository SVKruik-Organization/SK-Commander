document.addEventListener('DOMContentLoaded', () => {
    console.log("Frontend Script Complete.");

    const path = window.location.href.slice(-15);
    const style = "2px solid var(--accent-a);"
    console.log()

    if (path == "tend/index.html") {
        document.querySelector('.active-index').style.borderBottom = style
    } else if (path == "html/links.html") {

    } else if (path == "statistics.html") {

    }
});

function switchEye() {
    const input = document.getElementById('pincode');

    if (input.type == `password`) {
        input.type = `text`;
        document.getElementById('eye').className = `fa-solid fa-eye-slash`;
    } else {
        input.type = `password`;
        document.getElementById('eye').className = `fa-solid fa-eye`;
    };
};

function login() {
    const tag = document.getElementById('usertag');
    const pin = document.getElementById('pincode');

    if (tag.value.length < 7 || pin.value.length != 4) {
        console.log("Input Error")
    };
};
