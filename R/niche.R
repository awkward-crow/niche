# niche.R

# Source this file from the project root after `cargo build --release`.
# Override the library path via the NICHE_SO environment variable if needed.

local({
  so <- Sys.getenv("NICHE_SO", file.path("target", "release", "libniche.so"))
  dyn.load(so)
})

make_rng <- function(seed = 42) {
  structure(
    .Call("wrap__SeededRng__new", as.double(seed)),
    class = "niche_rng"
  )
}

make_walk <- function() {
  structure(
    .Call("wrap__Walk__new"),
    class = "niche_walk"
  )
}

step <- function(x, ...) UseMethod("step")

step.niche_walk <- function(walk, rng, ...) {
  .Call("wrap__Walk__step", walk, rng)
  invisible(walk)
}

length.niche_walk <- function(x) {
  .Call("wrap__Walk__length", x)
}

last <- function(x, ...) UseMethod("last")

last.niche_walk <- function(x, ...) {
  .Call("wrap__Walk__last", x)
}

path <- function(x, ...) UseMethod("path")

path.niche_walk <- function(x, ...) {
  .Call("wrap__Walk__path", x)
}


### end
