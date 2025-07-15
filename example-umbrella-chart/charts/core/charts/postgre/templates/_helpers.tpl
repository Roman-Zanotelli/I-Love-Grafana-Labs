{{- define "postgres.fullname" -}}
{{ .Release.Name }}-{{ .name }}
{{- end }}
{{- define "postgres.name" -}}
{{ .name }}
{{- end }}