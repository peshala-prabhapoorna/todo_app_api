fetch('./components/navbar/header.html')
.then(res => res.text())
.then(text => {
    let oldelem = document.querySelector("script#replace_with_navbar");
    let newelem = document.createElement("div");
    newelem.innerHTML = text;
    oldelem.parentNode.replaceChild(newelem,oldelem);
});


function activePageHighlight(anchorTab) {
    document.querySelectorAll(".nav-link").forEach(link => {
        link.classList.remove("active");
    })
    document.getElementById(anchorTab).classList.add("active");
}


function fetchContent(page) {
    const contentHtml = `./pages/${page}/${page}.html`;
    fetch(contentHtml)
        .then(resource => resource.text())
        .then(html => {
            document.getElementById("content_in_here").innerHTML = html;
        })
        .catch(error => {
            console.error(`Error fetching content: ${error}`);
        });
}


document.querySelectorAll(".nav-link").forEach(link => {
    console.log("i'm in add event");
    link.addEventListener("click", function(event){
        const targetId = event.currentTarget.id;
        activePageHighlight(targetId);
        fetchContent(targetId);
    })
})