// Progressive enhancement only: every page works without this file.
(() => {
  "use strict";

  // Branch pills: filter the mixed grid in place and update the address.
  const pills = document.querySelector("[data-pills]");
  const grid = document.querySelector(".card-grid");
  if (pills && grid && pills.querySelector("[data-filter]")) {
    const show = (value) => {
      for (const card of grid.children) {
        card.hidden = value !== "all" && card.dataset.group !== value;
      }
      for (const p of pills.querySelectorAll(".pill")) {
        if (p.dataset.filter === value) p.setAttribute("aria-current", "page");
        else p.removeAttribute("aria-current");
      }
    };
    pills.addEventListener("click", (e) => {
      const pill = e.target.closest("[data-filter]");
      if (!pill || e.metaKey || e.ctrlKey || e.shiftKey) return;
      e.preventDefault();
      show(pill.dataset.filter);
      history.pushState({ filter: pill.dataset.filter }, "", pill.href);
    });
    addEventListener("popstate", (e) => show(e.state?.filter ?? "all"));
  }

  // Music: show one of Artists / Albums / Tracks at a time.
  const facets = document.querySelector("[data-facets]");
  if (facets) {
    const sections = [...document.querySelectorAll(".facet")];
    const show = (id) => {
      if (!sections.some((s) => s.id === id)) id = sections[0].id;
      for (const s of sections) s.hidden = s.id !== id;
      for (const c of facets.querySelectorAll(".chip")) c.setAttribute("aria-pressed", String(c.dataset.facet === id));
    };
    document.body.classList.add("js-facets");
    facets.addEventListener("click", (e) => {
      const chip = e.target.closest("[data-facet]");
      if (!chip) return;
      e.preventDefault();
      show(chip.dataset.facet);
      history.replaceState(null, "", "#" + chip.dataset.facet);
    });
    show(location.hash.slice(1));
  }

  // Art: filter by group.
  const artFilter = document.querySelector("[data-art-filter]");
  if (artFilter) {
    artFilter.hidden = false;
    artFilter.addEventListener("click", (e) => {
      const chip = e.target.closest("[data-value]");
      if (!chip) return;
      const value = chip.dataset.value;
      for (const c of artFilter.querySelectorAll(".chip")) c.setAttribute("aria-pressed", String(c === chip));
      for (const f of document.querySelectorAll(".art")) f.hidden = value !== "all" && f.dataset.group !== value;
    });
  }

  // Lightbox for images in galleries, prose and the art grid.
  const scopes = document.querySelectorAll("[data-lightbox]");
  if (scopes.length) {
    const box = document.createElement("dialog");
    box.className = "lightbox";
    box.innerHTML =
      '<img alt=""><button type="button" class="lightbox__close" aria-label="Close">×</button>' +
      '<button type="button" class="lightbox__prev" aria-label="Previous">‹</button>' +
      '<button type="button" class="lightbox__next" aria-label="Next">›</button>';
    document.body.append(box);
    const img = box.querySelector("img");
    let set = [];
    let i = 0;
    const open = (n) => {
      i = (n + set.length) % set.length;
      img.src = set[i].currentSrc || set[i].src;
      img.alt = set[i].alt;
      if (!box.open) box.showModal();
    };
    for (const scope of scopes) {
      scope.addEventListener("click", (e) => {
        const target = e.target.closest("img");
        if (!target || target.closest("a")) return;
        set = [...scope.querySelectorAll("img")].filter((x) => !x.closest("a") && x.offsetParent);
        open(set.indexOf(target));
      });
    }
    box.querySelector(".lightbox__close").onclick = () => box.close();
    box.querySelector(".lightbox__prev").onclick = () => open(i - 1);
    box.querySelector(".lightbox__next").onclick = () => open(i + 1);
    box.addEventListener("click", (e) => { if (e.target === box) box.close(); });
    box.addEventListener("keydown", (e) => {
      if (e.key === "ArrowLeft") open(i - 1);
      if (e.key === "ArrowRight") open(i + 1);
    });
  }
})();
