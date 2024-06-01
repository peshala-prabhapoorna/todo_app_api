fetch('./components/navbar/header.html')
.then(res => res.text())
.then(text => {
    let oldelem = document.querySelector("script#replace_with_navbar");
    let newelem = document.createElement("div");
    newelem.innerHTML = text;
    oldelem.parentNode.replaceChild(newelem,oldelem);
})


const activePage = window.location.pathname;
const navLinks = document.querySelectorAll(".nav-link")
    .forEach(link => {
        if(link.href.includes(`${activePage}`)){
            link.classList.add('active');
            link.ariaCurrent.add('page');
        }
    })