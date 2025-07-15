{{- define "loki.fullname" -}}
{{ .Release.Name }}-loki
{{- end }}

{{- define "loki.labels" -}}
app.kubernetes.io/name: loki
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "loki.selectorLabels" -}}
app.kubernetes.io/name: loki
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}