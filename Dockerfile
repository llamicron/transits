FROM shepmaster/rust-nightly

RUN apt-get update

# -- Install Vartools --
# We need gnuplot, gcc, curl, make, etc..
RUN apt-get install gnuplot gcc curl make gfortran python -y

# Install cfitsio
WORKDIR /home
RUN git clone https://github.com/healpy/cfitsio.git
WORKDIR /home/cfitsio
RUN ./configure --prefix=/usr/local/
RUN make shared
RUN make install

WORKDIR /home
RUN curl https://www.astro.princeton.edu/~jhartman/vartools/vartools-1.33.tar.gz > vartools.tgz
RUN tar -xzvf vartools.tgz
# RUN git clone https://github.com/joeldhartman/vartools.git
RUN ls /home/
WORKDIR /home/vartools-1.33
RUN ./configure
RUN make && make install

# AbiusX on stackoverflow is a god
RUN ldconfig

RUN mkdir /home/astrotools
WORKDIR /home/astrotools

COPY . .
RUN cargo build --release

RUN mkdir /home/data
VOLUME [ "/home/data" ]

CMD ["astrotools"]
