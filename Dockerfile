FROM registry.scc.suse.de/sles12_base

RUN zypper --non-interactive addrepo http://download.suse.de/ibs/SUSE:/SLE-12:/GA/standard/SUSE:SLE-12:GA.repo
RUN zypper --gpg-auto-import-keys refresh
RUN zypper --non-interactive install wget

RUN wget http://username:password@gaffer.suse.de:9999/files/.regcode -O /root/.regcode
ADD target/debug/uconnect /usr/local/sbin/
