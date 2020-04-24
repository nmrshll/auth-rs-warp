package k8s

deployment <Name>: {
	apiVersion: *"extensions/v1beta1" | string
	kind:       "Deployment"
	metadata name: *Name | string
	spec: {
		replicas: *1 | int
		selector matchLabels app: *Name | string
		template: {
			metadata labels app: *Name | string
			spec containers: [{name: *Name | string }]
		}
	}
}
service <Name>: {
	apiVersion: *"v1" | string
	kind:       "Service"
	metadata name: *Name | string
	metadata annotations "dev.okteto.com/auto-ingress": "true"
	spec: {
		type: *"ClusterIP" | string
		selector app: *Name | string
		ports: [{
			name: *Name | string
			port: int
		}]
	}
}
configMap <Name>: {
	apiVersion: *"v1" | string
	kind: "ConfigMap"
	metadata name: *"\(Name)-config" | string
	metadata labels app: *Name | string
}
persistentVolumeClaim <Name>: {
	apiVersion: *"v1" | string
	kind: "PersistentVolumeClaim"
	metadata name: *"\(Name)-pv-claim" | string
	spec accessModes: ["ReadWriteOnce"]
	spec resources requests storage: *"10Gi" | string
}
job <Name>: {
	apiVersion: *"batch/v1" | string
	kind: "Job"
	metadata name: *Name | string
	spec template spec restartPolicy: *"Never" | string
	spec template spec containers: [{ name: *Name | string }]
	spec backoffLimit: *4 | int
}


//////////////////


deployment api: {
	spec template spec containers: [{
		image: "registry.gitlab.com/nmrshll-weekend-projects/notajobboard-api-hyper:latest"
		envFrom: [{ secretRef name: "notajobboard-postgres-db" }]
	}]
	spec template spec imagePullSecrets:[{
		// manually: add credentials to cluster: https://kubernetes.io/docs/tasks/configure-pod-container/pull-image-private-registry/
		name: "notajobboard-api-hyper-regcred"
	}]
}
service api: {
	spec ports: [{ port: 8080 }]
}

deployment postgres: {
	spec template spec containers: [{
		image: "postgres:10.4"
		ports: [{ containerPort: 5432 }]
		envFrom: [{ secretRef name: "notajobboard-postgres-db" }]
		volumeMounts: [{ 
			name:  "postgres-data-volume"
			mountPath: "/var/lib/postgresql/data"
			subPath: "data"
		}]
	}]
	spec template spec volumes: [{
		name: "postgres-data-volume"
		persistentVolumeClaim claimName: "postgres-pv-claim"
	}]
}
service postgres: {
	spec ports: [{ port: 5432 }]
}
// 	configMap postgres data: {
// 		POSTGRES_DB: "postgresdb"
// 		POSTGRES_USER: "postgresuser"
// 		POSTGRES_PASSWORD: "postgrespassword"
// 	}
persistentVolumeClaim postgres: {}

deployment adminer: {
	spec template spec containers: [{
		image: "adminer:4.2.5"
		ports: [{ containerPort: 8080 }]
	}]
}
service adminer: {
	spec ports: [{
		port: 7897
		targetPort: 8080
	}]
}

job migrations: {
	spec template spec containers: [{
		image: "registry.gitlab.com/nmrshll-weekend-projects/notajobboard-api-hyper:migr_latest"
		envFrom: [{ secretRef name: "notajobboard-postgres-db" }]
	}]
	spec template spec imagePullSecrets:[{ name: "notajobboard-api-hyper-regcred" }]
}





