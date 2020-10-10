.PHONY: all $(MAKECMDGOALS)
.DEFAULT_GOAL=dev
dev: deps pg adminer migrate
	${pg_dsn} cargo watch -x run
build:
	docker build -f .config/deploy/build.Dockerfile -t ${cwd} . 
test: down deps pg migrate
	${pg_dsn} cargo test -- --nocapture


# REQUESTS
/:
	curl ${addr}/
protected:
	curl ${post} ${addr}/protected/ -d '{}' -b token=AAAAAF3avy4AAAAAXdwQrgAAAAAAAAABZvLHEexqQgbaTTniiMEqSGlWfoAUPEpfSXRMwQErfrF05DCvVOAFO1JaC/9bld5K4xmfvlwT/cs5FQNVJ7ll6A==
users/login:
	curl ${post} ${addr}/users/login -i -d '{"email":"user.email@gmail.com", "password":"nopass"}'
users/check:
	curl ${post} ${addr}/users/check -d '{"email":"user.email@gmail.com"}'
users/register:
	curl ${post} ${addr}/users/register -d '{"email":"user.email@gmail.com","password":"nopass"}'
addr=$(if $(filter $(ENV),P),https://api-nmrshll.cloud.okteto.net,http://0.0.0.0:8080)
post= -X POST -H "Content-Type: application/json"


# CONTAINERS
api: pull 
	$(eval srvc=api) ${(re)launchContainer} -p 0.0.0.0:8080:8080 -d hello-world
pg:
	$(eval srvc=pg) ${(re)launchContainer} -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD=docker -e POSTGRES_USER=docker -e POSTGRES_DB=docker -d postgres:alpine
adminer: 
	$(eval srvc=adminer) ${(re)launchContainer} -d -p 127.0.0.1:7897:8080 adminer:4.2.5
migrate: 
	@$(eval SHELL:=/bin/bash) while ! test "`echo -ne "\x00\x00\x00\x17\x00\x03\x00\x00user\x00username\x00\x00" | nc -w 3 127.0.0.1 5432 2>/dev/null | head -c1`" = R; do echo "waiting on postgres..."; sleep 0.3; done;
	${pg_dsn} diesel migration run	
migrate2:
	docker run --rm \
    -w /workdir -v $(shell pwd)/migrations:/workdir/migrations \
    -e DATABASE_URL="postgres://docker:docker@host.docker.internal/docker" \
    -it clux/diesel-cli diesel migration run
down:
	-docker rm -f -v `docker ps -a -q --filter "name=${cwd}"`
logs: 
	$(eval srvc=api) docker logs -f ${container_name}
cwd = $(notdir $(shell pwd))
container_name = ${cwd}-${srvc}
ifContainerMissing = @docker container inspect ${container_name} > /dev/null 2>&1 || 
(re)launchContainer = ${ifContainerMissing} docker run --rm --name ${container_name} -v $(shell pwd)/.config:/config:ro --stop-signal "SIGINT"
pg_dsn=DATABASE_URL=postgres://docker:docker@127.0.0.1/docker
pull:
	docker pull hello-world

# K8S
k.all: k.api k.pg k.adminer
k.api: dirs
	-$(call cueDo,deployment.api)
	-$(call cueDo,service.api)
k.pg: dirs
	-$(call cueDo,deployment.postgres)
	-$(call cueDo,service.postgres)
	-$(call cueDo,configMap.postgres)
	-$(call cueDo,persistentVolumeClaim.postgres)
k.adminer: dirs
	-$(call cueDo,deployment.adminer)
	-$(call cueDo,service.adminer)
k.migr: dirs
	-$(call cueDo,job.migrations)
cueDo = cue eval -e $(1) deploy/k8s.dply.cue > .cache/k8s/$(1).cue; cue export .cache/k8s/$(1).cue | kubectl $(if $(filter $(DEL),1),delete,apply) -f -
k.regcred: # manually: write github personal access token to file
	kubectl create secret docker-registry regcred-${cwd} --docker-server=docker.pkg.github.com --docker-username=nmrshll --docker-password=`cat ~/.config/github/TOKEN_read:packages`


# DEPS
deps: dirs
	@rustc --version | grep -E 'nightly.*2020-04-23' $s || rustup override set nightly-2020-04-23
	@drill --version | grep 0.5.0 $s || cargo install drill --version 0.5.0
dirs:
	@mkdir -p .cache/k8s/
installs: 		# install manually: docker, build-essential, pkg-config
	@rustup --version $s || curl https://sh.rustup.rs -sSf | sh -s -- -y
s = &>/dev/null

